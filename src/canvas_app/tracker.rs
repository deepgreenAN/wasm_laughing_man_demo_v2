mod tracker_roi;

use rustface::Rectangle;
use tracker_roi::{RoiState, TrackerRoi};

const INF: f64 = 10_i32.pow(5) as f64;
pub struct Tracker {
    pub rois: Vec<TrackerRoi>,
    id_counter: u32,
    pub allowable_not_detect_count: u32,
}

impl Tracker {
    pub fn new(allowable_not_detect_count: u32) -> Self {
        Self {
            rois: Vec::new(),
            id_counter: 0,
            allowable_not_detect_count,
        }
    }
    pub fn ids(&self) -> Vec<u32> {
        self.rois.iter().map(|roi| roi.id).collect::<Vec<u32>>()
    }
    pub fn track(&mut self, faces: &Vec<Rectangle>) -> (Vec<TrackerRoi>, Vec<TrackerRoi>) {
        let mut added_rois: Vec<TrackerRoi> = Vec::new();
        let mut removed_rois: Vec<TrackerRoi> = Vec::new();

        let mut face_roi_conneted_flags: Vec<bool> = vec![false; faces.len()];
        // roisとfacesの結び付け(割り当て問題として解かずにidの小さいものから結びつける(貪欲法))
        let mut min_dist_index: Option<usize> = None;
        let mut min_dist = INF;
        for roi in self.rois.iter_mut() {
            faces
                .iter()
                .enumerate()
                .filter(|(i, _)| !face_roi_conneted_flags[*i])
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
        let rois_length = self.rois.len();
        let mut new_rois: Vec<TrackerRoi> = Vec::new();
        for i in 0..rois_length {
            let roi = self.rois[i].clone();
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
        self.rois = new_rois;

        //roiの追加
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
}
