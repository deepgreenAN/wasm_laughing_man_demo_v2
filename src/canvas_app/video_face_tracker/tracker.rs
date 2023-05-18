use super::tracker_roi::{RoiState, TrackerRoi};
use rustface::Rectangle;

use std::cmp::Ordering;

/// シンプルな矩形のトラッカー
pub struct Tracker {
    pub rois: Vec<TrackerRoi>,
    id_counter: u32,
    pub allowable_not_detect_count: u32,
}

impl Tracker {
    /// コンストラクタ
    pub fn new(allowable_not_detect_count: u32) -> Self {
        Self {
            rois: Vec::new(),
            id_counter: 0,
            allowable_not_detect_count,
        }
    }
    /// 状態を一つ遷移させてトラッキング
    /// Results
    /// - 追加されたRoiの配列
    /// - 削除されたRoiの配列
    pub fn track(&mut self, faces: &Vec<Rectangle>) -> (Vec<TrackerRoi>, Vec<TrackerRoi>) {
        let mut face_roi_connected_flags: Vec<bool> = vec![false; faces.len()];
        // roisとfacesの結び付け(割り当て問題として解かずにidの小さいものから最も使いものと結びつける(貪欲法))
        for roi in self.rois.iter_mut() {
            let min_dist_face_and_flag_opt = faces
                .iter()
                .zip(face_roi_connected_flags.iter_mut())
                .filter(|(_, flag)| !**flag) // まだマッチングしていないもののみ
                .min_by(|(face_x, _), (face_y, _)| {
                    match roi
                        .distance_with_rect(*face_x)
                        .partial_cmp(&roi.distance_with_rect(*face_y))
                    {
                        Some(ordering) => ordering,
                        None => {
                            Ordering::Greater // 計算できない(NaN)なら大きいとして最小は無いようにする
                        }
                    }
                });

            if let Some((min_dist_face, flag)) = min_dist_face_and_flag_opt {
                // マッチングした場合
                *flag = true;
                roi.tl_x = min_dist_face.x() as f64;
                roi.tl_y = min_dist_face.y() as f64;
                roi.width = min_dist_face.width() as f64;
                roi.height = min_dist_face.height() as f64;

                roi.detected();
            } else {
                roi.not_detected();
            }
        }

        //roiの削除
        let mut removed_rois: Vec<TrackerRoi> = Vec::new();

        self.rois = {
            let mut new_rois = Vec::<TrackerRoi>::new();

            let rois = std::mem::take(&mut self.rois);

            for roi in rois.into_iter() {
                match roi.state {
                    RoiState::DETECTED => {
                        new_rois.push(roi);
                    }
                    RoiState::NOTDETECTED => {
                        if roi.not_detected_count <= self.allowable_not_detect_count {
                            new_rois.push(roi);
                        } else {
                            removed_rois.push(roi);
                        }
                    }
                }
            }
            new_rois
        };

        //roiの追加
        let mut added_rois = Vec::<TrackerRoi>::new();

        for (new_face, _) in faces
            .iter()
            .zip(face_roi_connected_flags.iter())
            .filter(|(_, flag)| !**flag)
        // マッチングしていないみののみ
        {
            self.id_counter += 1;
            let roi = TrackerRoi::new(
                self.id_counter,
                new_face.x() as f64,
                new_face.y() as f64,
                new_face.width() as f64,
                new_face.height() as f64,
            );

            added_rois.push(roi.clone());
            self.rois.push(roi);
        }

        (added_rois, removed_rois)
    }

    pub fn rois(&self) -> &[TrackerRoi] {
        &self.rois
    }
}
