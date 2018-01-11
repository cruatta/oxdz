use module::Module;
use format::FormatPlayer;
use player::{PlayerData, Virtual};
use super::ModPatterns;

const FX_TONEPORTA: u8 = 0x03;

pub struct ModPlayer {
    name : &'static str,
    state: Vec<ChannelData>,

//  mt_speed          : u8,  // -> data.speed
//  mt_counter        : u8,  // -> data.frame
//  mt_song_pos       : u8,  // -> data.pos
    mt_pbreak_pos     : u8,
    mt_pos_jump_flag  : u8,
    mt_pbreak_flag    : u8,
    mt_low_mask       : u8,
    mt_patt_del_time  : u8,
    mt_patt_del_time_2: u8,
}

impl ModPlayer {
    pub fn new(module: &Module) -> Self {
        ModPlayer {
            name : r#""Vinterstigen" 0.1 PT2.1A replayer"#,
            state: vec![ChannelData::new(); module.chn],

//          mt_speed          : 0,
//          mt_counter        : 0,
//          mt_song_pos       : 0,
            mt_pbreak_pos     : 0,
            mt_pos_jump_flag  : 0,
            mt_pbreak_flag    : 0,
            mt_low_mask       : 0,
            mt_patt_del_time  : 0,
            mt_patt_del_time_2: 0,
        }
    }

    fn mt_get_new_note(&mut self, mut data: &mut PlayerData, module: &Module, pats: &ModPatterns, virt: &mut Virtual) {
        for chn in 0..self.state.len() {
            // mt_PlayVoice
            let event = pats.event(data.pos, data.row, chn);
            if event.has_ins() {
                let instrument = &module.instrument[event.ins as usize];
                virt.set_patch(chn, event.ins as usize, event.ins as usize, event.note as usize);
                virt.set_volume(chn, instrument.volume);
            }

            // mt_SetRegs
            if event.has_note() {

                let period = 100_f64;

                match event.cmd {
                    0xe => if (event.cmdlo & 0xf0) == 0x50 {
                                // mt_DoSetFinetune()
                           },
                    0x3 => { self.mt_set_tone_porta(chn, &mut data); self.mt_check_efx(chn, &mut data) },
                    0x5 => { self.mt_set_tone_porta(chn, &mut data); self.mt_check_efx(chn, &mut data) },
                    0x9 => { self.mt_check_more_efx(chn, &mut data); virt.set_period(chn, period) },
                    _   => virt.set_period(chn, period),
                }
                

            } else {
                self.mt_check_more_efx(chn, &mut data);
            }
        }
    }

    fn mt_check_efx(&mut self, chn: usize, mut data: &mut PlayerData) {
        let cmd = 0;

        // mt_UpdateFunk()
        if cmd == 000 {
            self.per_nop(chn, &mut data);
            return
        }

        match cmd {
            0x0 => self.mt_arpeggio(chn, &mut data),
            0x1 => self.mt_porta_up(chn, &mut data),
            0x2 => self.mt_porta_down(chn, &mut data),
            0x3 => self.mt_tone_portamento(chn, &mut data),
            0x4 => self.mt_vibrato(chn, &mut data),
            0x5 => self.mt_tone_plus_vol_slide(chn, &mut data),
            0x6 => self.mt_vibrato_plus_vol_slide(chn, &mut data),
            0xe => self.mt_e_commands(chn, &mut data),
// SetBack MOVE.W  n_period(A6),6(A5)
            0x7 => self.mt_tremolo(chn, &mut data),
            0xa => self.mt_volume_slide(chn, &mut data),
            _   => {},
        }
    }

    fn per_nop(&self, chn: usize, mut data: &mut PlayerData) {
        //self.state.n_period = period
    }

    fn mt_arpeggio(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_fine_porta_up(&mut self, chn: usize, mut data: &mut PlayerData) {
        if data.frame != 0 {
            return
        }
        self.mt_low_mask = 0x0f;
        self.mt_porta_up(chn, &mut data);
    }

    fn mt_porta_up(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_fine_porta_down(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_porta_down(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_set_tone_porta(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_clear_tone_porta(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_tone_portamento(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_vibrato(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_tone_plus_vol_slide(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_vibrato_plus_vol_slide(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_tremolo(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_sample_offset(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_volume_slide(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_position_jump(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_volume_change(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_pattern_break(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_set_speed(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_check_more_efx(&mut self, chn: usize, mut data: &mut PlayerData) {
        let cmd = 0;

        // mt_UpdateFunk()
        match cmd {
            0x9 => self.mt_sample_offset(chn, &mut data),
            0xb => self.mt_position_jump(chn, &mut data),
            0xd => self.mt_pattern_break(chn, &mut data),
            0xe => self.mt_e_commands(chn, &mut data),
            0xf => self.mt_set_speed(chn, &mut data),
            0xc => self.mt_volume_change(chn, &mut data),
            _   => {},
        }

        // per_nop
        self.per_nop(chn, &mut data)
    }

    fn mt_e_commands(&mut self, chn: usize, mut data: &mut PlayerData) {
        let cmd = 0;

        match cmd {
           0x0 => self.mt_filter_on_off(chn, &mut data),
           0x1 => self.mt_fine_porta_up(chn, &mut data),
           0x2 => self.mt_fine_porta_down(chn, &mut data),
           0x3 => self.mt_set_gliss_control(chn, &mut data),
           0x4 => self.mt_set_vibrato_control(chn, &mut data),
           0x5 => self.mt_set_finetune(chn, &mut data),
           0x6 => self.mt_jump_loop(chn, &mut data),
           0x7 => self.mt_set_tremolo_control(chn, &mut data),
           0x9 => self.mt_retrig_note(chn, &mut data),
           0xa => self.mt_volume_fine_up(chn, &mut data),
           0xb => self.mt_volume_fine_down(chn, &mut data),
           0xc => self.mt_note_cut(chn, &mut data),
           0xd => self.mt_note_delay(chn, &mut data),
           0xe => self.mt_pattern_delay(chn, &mut data),
           0xf => self.mt_funk_it(chn, &mut data),
           _   => {},
        }
    }

    fn mt_filter_on_off(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_set_gliss_control(&self, chn: usize, mut data: &mut PlayerData) {
        //state.n_glissfunk = parm;
    }

    fn mt_set_vibrato_control(&self, chn: usize, mut data: &mut PlayerData) {
        //state.n_wavecontrol &= 0xf0;
        //state.n_wavecontrol |= parm & 0x0f;
    }

    fn mt_set_finetune(&self, chn: usize, mut data: &mut PlayerData) {
        //state.n_finetune = parm;
    }

    fn mt_jump_loop(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_set_tremolo_control(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_retrig_note(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_volume_fine_up(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_volume_fine_down(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_note_cut(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_note_delay(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_pattern_delay(&self, chn: usize, mut data: &mut PlayerData) {
    }

    fn mt_funk_it(&self, chn: usize, mut data: &mut PlayerData) {
    }
}

impl FormatPlayer for ModPlayer {
    fn name(&self) -> &'static str {
        self.name
    }

    fn play(&mut self, mut data: &mut PlayerData, module: &Module, mut virt: &mut Virtual) {
        let pats = module.patterns.as_any().downcast_ref::<ModPatterns>().unwrap();

        data.frame += 1;
        if data.frame >= data.speed {
            data.frame = 0;
            if self.mt_patt_del_time_2 == 0 {
                self.mt_get_new_note(&mut data, &module, &pats, &mut virt);
            } else {
                // mt_NoNewAllChannels
                for chn in 0..self.state.len() {
                    self.mt_check_efx(chn, &mut data)
                }
		


                // mt_NoNewPosYet
            }
        }
    }

    fn reset(&mut self) {
        self.mt_pbreak_pos      = 0;
        self.mt_pos_jump_flag   = 0;
        self.mt_pbreak_flag     = 0;
        self.mt_low_mask        = 0;
        self.mt_patt_del_time   = 0;
        self.mt_patt_del_time_2 = 0;
    }
}


#[derive(Clone,Default)]
struct ChannelData {
    n_note         : u8,
    n_cmd          : u8,
    n_cmdlo        : u8,
    n_period       : u16,
    n_finetune     : i8,
    n_volume       : u8,
    n_toneportdirec: i8,
    n_toneportspeed: u8,
    n_wantedperiod : u16,
    n_vibratocmd   : u8,
    n_vibratopos   : u8,
    n_tremolocmd   : u8,
    n_tremolopos   : u8,
    n_wavecontrol  : u8,
    n_glissfunk    : u8,
    n_sampleoffset : u8,
    n_pattpos      : u8,
    n_loopcount    : u8,
    n_funkoffset   : u8,
    n_wavestart    : u32,
    n_reallength   : u16,
}

impl ChannelData {
    pub fn new() -> Self {
        Default::default()
    }
}
