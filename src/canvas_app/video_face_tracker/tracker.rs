use super::tracker_roi::{RoiState, TrackerRoi};
use rustface::Rectangle;

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
        let mut face_roi_conneted_flags: Vec<bool> = vec![false; faces.len()];
        // roisとfacesの結び付け(割り当て問題として解かずにidの小さいものから最も使いものと結びつける(貪欲法))
        let mut min_dist_index: Option<usize> = None;
        let mut min_dist = f64::INFINITY;
        for roi in self.rois.iter_mut() {
            faces
                .iter()
                .enumerate()
                .filter(|(i, _)| !face_roi_conneted_flags[*i]) // まだ結びついていないもののみになるようにフィルタリング
                .for_each(|(i, rect)| {
                    let dist = roi.distance_with_rect(rect);
                    if dist < min_dist {
                        min_dist_index = Some(i);
                        min_dist = dist;
                    }
                });
            if let Some(min_dist_index) = min_dist_index {
                // マッチングした場合
                face_roi_conneted_flags[min_dist_index] = true; // 結びついたのでフラッグを更新
                roi.tl_x = faces[min_dist_index].x() as f64;
                roi.tl_y = faces[min_dist_index].y() as f64;
                roi.width = faces[min_dist_index].width() as f64;
                roi.height = faces[min_dist_index].height() as f64;

                roi.detected();
            } else {
                // マッチングしなかった場合
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

        faces
            .iter()
            .enumerate()
            .filter(|(i, _)| !face_roi_conneted_flags[*i])
            .for_each(|(_, rect)| {
                self.id_counter += 1;
                let roi = TrackerRoi::new(
                    self.id_counter,
                    rect.x() as f64,
                    rect.y() as f64,
                    rect.width() as f64,
                    rect.height() as f64,
                );
                added_rois.push(roi.clone());
                self.rois.push(roi);
            });

        (added_rois, removed_rois)
    }

    pub fn rois(&self) -> &[TrackerRoi] {
        &self.rois
    }
}
