mod player;

use module::Module;
use player::{Options, PlayerListEntry, PlayerInfo, FormatPlayer};

pub struct Nt11;

impl PlayerListEntry for Nt11 {
   fn info(&self) -> PlayerInfo {
       PlayerInfo {
          id         : "nt11",
          name       : "Noiseplay11 NT V1.1 replayer",
          description: "A mod player based on the on the original Noisetracker V1.1 replayer",
          author     : "Claudio Matsuoka",
          accepts    : &[ "m.k." ],
       }
   }

   fn player(&self, module: &Module, options: Options) -> Box<FormatPlayer> {
       Box::new(self::player::ModPlayer::new(module, options))
   }
}


