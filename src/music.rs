use std::vec::Vec;

struct Notes {
    fund_frequencies: Vec<[f32; 2]>,
}

impl Notes {
    pub fn new() -> Self {
        Self {
            fund_frequencies: std::vec![
                ([15.835, 16.835]), // C0
                ([16.836, 17.805]), // C#0 (Db0)
                ([17.806, 18.895]), // D0
                ([18.896, 20.005]), // D#0 (Eb0)
                ([20.006, 21.195]), // E0
                ([21.196, 22.465]), // F0
                ([22.466, 23.775]), // F#0 (Gb0)
                ([23.776, 25.225]), // G0
                ([25.226, 26.695]), // G#0 (Ab0)
                ([26.696, 28.305]), // A0
                ([28.306, 29.975]), // A#0 (Bb0)
                ([29.976, 31.765]), // B0
            ]
        }
    }

    pub fn get_note(&self, frequency: f32) -> String {
        // let bucket_idx = 
    }
}



