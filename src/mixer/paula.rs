
// 131072 to 0, 2048 entries
pub const PAULA_HZ        : f64   = 3546895.0;
pub const MINIMUM_INTERVAL: usize = 16;

const BLEP_SCALE: u32   = 17;
const BLEP_SIZE : usize = 2048;
const MAX_BLEPS : usize = (BLEP_SIZE / MINIMUM_INTERVAL);

// the structure that holds data of bleps
#[derive(Copy,Clone,Default)]
struct Blep {
    level: i16,
    age  : i16,
}

#[derive(Clone)]
pub struct Paula {
    // the instantenous value of Paula output
    global_output_level: i16,

    // count of simultaneous bleps to keep track of
    active_bleps: usize,

    // place to keep our bleps in. MAX_BLEPS should be
    // defined as a BLEP_SIZE / MINIMUM_EVENT_INTERVAL.
    // For Paula, minimum event interval could be even 1, but it makes
    // sense to limit it to some higher value such as 16.
    bleps: [Blep; MAX_BLEPS],

    pub remainder: f64,
    pub fdiv     : f64,
}

impl Paula {
    pub fn new(freq: u32) -> Self {
        Paula{
            global_output_level: 0,
            active_bleps       : 0,
            bleps              : [Default::default(); MAX_BLEPS],
            fdiv               : PAULA_HZ / freq as f64,
            remainder          : PAULA_HZ / freq as f64,
        }
    }

    // return output simulated as series of bleps
    pub fn output_sample(&self, tabnum: usize) -> i16 {
        let mut output = (self.global_output_level as i32) << BLEP_SCALE;
        for i in 0..self.active_bleps {
            let age = self.bleps[i].age as usize;
            let level = self.bleps[i].level as i32;
            output -= WINSINC_INTEGRAL[tabnum][age] * level;
        }
        output >>= BLEP_SCALE;
        clamp!(output, -32768, 32767);
    
        output as i16
    }
 
    pub fn input_sample(&mut self, sample: i16) {
        if sample != self.global_output_level {
            // Start a new blep: level is the difference, age (or phase) is 0 clocks.
            if self.active_bleps > MAX_BLEPS - 1 {
                println!("warning: active blep list truncated!");
                self.active_bleps = MAX_BLEPS - 1;
            }
    
            // Make room for new blep
            for i in 0..self.active_bleps {
                let index = self.active_bleps - i;
                self.bleps[index].age = self.bleps[index-1].age;
                self.bleps[index].level = self.bleps[index-1].level;
            }
    
            // Update state to account for the new blep
            self.active_bleps += 1;
            self.bleps[0].age = 0;
            self.bleps[0].level = sample - self.global_output_level;
            self.global_output_level = sample;
        }
    }
    
    pub fn do_clock(&mut self, cycles: i16) {
        if cycles <= 0 {
            return
        }
    
        for i in 0..self.active_bleps {
            self.bleps[i].age += cycles;
            if self.bleps[i].age >= BLEP_SIZE as i16 {
                self.active_bleps = i;
                break
            }
        }
    }
}


//
// Table generated by compute-blep.py (a1200 and vanilla tables removed)
//

// tables are: a500 off, a500 on
static WINSINC_INTEGRAL: [[i32; 2048]; 2] = [
    [
        131072,131072,131072,131072,131072,131072,131072,131072,131072,131072,131072,
        131072,131072,131072,131072,131072,131072,131072,131072,131072,131072,131071,131071,
        131071,131071,131071,131071,131071,131071,131071,131071,131071,131070,131070,131070,
        131070,131070,131069,131069,131069,131068,131068,131068,131067,131067,131066,131066,
        131065,131065,131064,131063,131063,131062,131061,131060,131059,131058,131056,131055,
        131054,131052,131050,131049,131047,131045,131043,131040,131038,131035,131033,131030,
        131026,131023,131020,131016,131012,131008,131003,130998,130993,130988,130982,130976,
        130970,130963,130956,130949,130941,130932,130924,130914,130905,130895,130884,130872,
        130861,130848,130835,130821,130807,130792,130776,130759,130742,130724,130705,130685,
        130664,130642,130620,130596,130571,130545,130518,130490,130461,130430,130398,130365,
        130331,130295,130257,130219,130178,130136,130093,130047,130000,129951,129901,129848,
        129794,129737,129679,129618,129555,129490,129423,129353,129281,129207,129130,129050,
        128968,128883,128795,128704,128611,128514,128415,128312,128206,128097,127985,127869,
        127750,127627,127501,127371,127237,127100,126959,126813,126664,126510,126353,126191,
        126025,125854,125679,125499,125315,125126,124933,124734,124531,124323,124110,123891,
        123668,123439,123205,122965,122720,122470,122214,121952,121685,121412,121133,120849,
        120558,120261,119959,119650,119335,119014,118687,118354,118014,117668,117315,116956,
        116591,116219,115840,115455,115063,114665,114260,113849,113430,113005,112574,112135,
        111690,111239,110780,110315,109843,109364,108879,108387,107888,107383,106871,106352,
        105827,105295,104757,104212,103661,103104,102540,101970,101394,100812,100223,99629,
        99028,98422,97810,97192,96568,95939,95305,94665,94020,93370,92714,92054,91389,90719,
        90045,89366,88682,87995,87303,86607,85908,85205,84498,83788,83075,82358,81639,80916,
        80191,79464,78734,78002,77268,76533,75795,75056,74316,73575,72833,72090,71346,70602,
        69858,69114,68370,67626,66883,66140,65399,64658,63919,63181,62445,61711,60979,60249,
        59521,58796,58074,57355,56639,55926,55217,54512,53810,53113,52419,51731,51046,50367,
        49693,49023,48359,47701,47048,46400,45759,45124,44495,43872,43256,42646,42043,41447,
        40858,40276,39702,39134,38575,38023,37478,36941,36413,35892,35379,34874,34378,33890,
        33410,32938,32475,32020,31574,31137,30708,30288,29876,29473,29079,28693,28317,27948,
        27589,27238,26896,26562,26238,25921,25613,25314,25023,24740,24466,24200,23942,23692,
        23451,23217,22991,22773,22562,22359,22164,21975,21794,21621,21454,21294,21140,20994,
        20853,20719,20592,20470,20354,20244,20139,20040,19946,19857,19774,19694,19620,19550,
        19484,19422,19364,19310,19260,19213,19169,19128,19090,19054,19022,18991,18963,18936,
        18912,18889,18867,18847,18828,18810,18792,18776,18759,18743,18727,18711,18695,18679,
        18662,18644,18626,18607,18587,18565,18542,18518,18492,18465,18436,18404,18371,18336,
        18298,18259,18216,18172,18124,18074,18022,17966,17908,17847,17783,17716,17646,17572,
        17496,17416,17334,17248,17159,17066,16971,16872,16770,16664,16556,16444,16329,16211,
        16090,15966,15839,15709,15576,15440,15301,15159,15015,14868,14718,14566,14412,14255,
        14096,13935,13771,13606,13439,13270,13099,12927,12753,12578,12401,12224,12045,11866,
        11685,11504,11322,11140,10958,10775,10592,10409,10226,10044,9862,9680,9499,9319,9139,
        8961,8783,8607,8432,8258,8086,7915,7747,7580,7415,7252,7091,6932,6776,6622,6471,
        6322,6176,6032,5892,5754,5619,5488,5359,5234,5111,4992,4877,4764,4655,4550,4448,
        4349,4254,4163,4075,3990,3910,3832,3759,3689,3622,3560,3500,3445,3393,3344,3299,
        3257,3219,3184,3153,3124,3099,3078,3059,3044,3031,3022,3015,3011,3010,3012,3016,
        3023,3033,3044,3058,3075,3093,3113,3136,3160,3186,3213,3242,3273,3305,3338,3372,
        3408,3444,3481,3520,3558,3597,3637,3677,3718,3758,3799,3839,3880,3920,3960,4000,
        4039,4077,4115,4152,4188,4224,4258,4291,4323,4354,4384,4412,4439,4464,4488,4510,
        4530,4549,4566,4581,4594,4606,4615,4623,4628,4631,4633,4632,4629,4624,4617,4608,
        4597,4583,4568,4550,4530,4508,4484,4458,4429,4399,4366,4332,4296,4257,4217,4175,
        4130,4085,4037,3988,3937,3884,3830,3774,3717,3658,3598,3537,3475,3411,3347,3281,
        3215,3147,3079,3010,2940,2870,2799,2728,2657,2585,2513,2440,2368,2296,2224,2151,
        2080,2008,1937,1866,1796,1726,1657,1589,1521,1454,1389,1324,1260,1197,1135,1075,
        1016,958,901,846,792,740,689,640,592,546,502,459,419,379,342,307,273,241,211,183,
        156,132,109,88,69,52,37,24,12,2,-5,-11,-16,-18,-19,-18,-16,-11,-6,2,11,21,33,47,61,
        77,95,113,133,154,176,200,224,249,275,302,329,358,387,416,447,477,508,540,572,604,
        636,669,702,734,767,800,832,864,896,928,960,991,1021,1051,1081,1110,1138,1166,1193,
        1219,1245,1270,1293,1316,1338,1359,1379,1398,1416,1433,1448,1463,1476,1488,1499,
        1509,1518,1525,1531,1536,1540,1542,1543,1543,1542,1539,1536,1530,1524,1517,1508,
        1498,1487,1475,1462,1447,1432,1415,1397,1379,1359,1338,1317,1294,1271,1247,1222,
        1196,1170,1143,1115,1086,1057,1028,998,967,936,905,874,842,809,777,744,712,679,646,
        613,581,548,515,483,450,418,387,355,324,293,263,233,204,175,147,119,92,66,40,15,
        -10,-33,-56,-78,-99,-120,-139,-158,-176,-193,-209,-224,-238,-252,-264,-275,-286,
        -295,-304,-311,-318,-324,-329,-332,-335,-337,-338,-338,-338,-336,-333,-330,-326,
        -321,-315,-308,-301,-293,-284,-274,-264,-253,-242,-229,-217,-203,-190,-175,-161,
        -145,-130,-114,-98,-81,-64,-47,-29,-11,6,24,42,61,79,97,115,134,152,170,188,206,223,
        241,258,275,291,308,324,340,355,370,384,399,412,425,438,450,462,473,484,494,503,
        512,521,529,536,542,548,553,558,562,566,568,570,572,573,573,573,572,570,568,565,
        562,558,553,548,543,537,530,523,515,507,498,489,479,469,459,448,437,426,414,402,
        389,377,364,351,337,324,310,296,282,268,254,239,225,211,196,182,168,153,139,125,
        111,97,83,70,56,43,30,17,5,-7,-19,-31,-42,-53,-64,-75,-85,-94,-104,-113,-121,-129,
        -137,-144,-151,-158,-164,-170,-175,-180,-184,-188,-192,-195,-198,-200,-202,-203,
        -204,-205,-205,-204,-204,-203,-201,-199,-197,-195,-192,-188,-185,-181,-176,-172,
        -167,-162,-156,-151,-145,-139,-132,-126,-119,-112,-104,-97,-90,-82,-74,-66,-59,-51,
        -42,-34,-26,-18,-10,-2,7,15,23,31,39,47,54,62,70,77,85,92,99,106,112,119,125,131,
        137,143,148,154,159,163,168,172,176,180,183,187,190,192,195,197,199,201,202,203,
        204,205,205,205,205,204,203,202,201,200,198,196,194,192,189,186,183,180,177,173,
        169,165,161,157,153,148,143,139,134,129,124,119,113,108,103,97,92,86,81,75,70,64,
        59,53,48,42,37,32,26,21,16,11,6,1,-4,-9,-13,-18,-22,-26,-30,-34,-38,-42,-46,-49,
        -52,-55,-58,-61,-64,-66,-69,-71,-73,-74,-76,-78,-79,-80,-81,-82,-82,-83,-83,-83,
        -83,-83,-83,-82,-82,-81,-80,-79,-78,-77,-75,-74,-72,-70,-68,-66,-64,-62,-60,-57,
        -55,-52,-50,-47,-44,-42,-39,-36,-33,-30,-27,-24,-21,-18,-15,-12,-9,-6,-3,0,3,6,8,
        11,14,17,20,22,25,27,30,32,35,37,39,41,43,45,47,49,50,52,54,55,56,58,59,60,61,61,
        62,63,63,64,64,65,65,65,65,65,64,64,64,63,63,62,61,61,60,59,58,57,56,55,53,52,51,
        49,48,46,45,43,41,40,38,36,35,33,31,29,28,26,24,22,20,19,17,15,13,11,10,8,6,5,3,1,
        0,-2,-3,-5,-6,-8,-9,-10,-12,-13,-14,-15,-16,-17,-18,-19,-20,-21,-21,-22,-23,-23,
        -24,-24,-25,-25,-26,-26,-26,-26,-26,-26,-26,-26,-26,-26,-26,-26,-25,-25,-25,-24,
        -24,-23,-23,-22,-21,-21,-20,-19,-19,-18,-17,-16,-16,-15,-14,-13,-12,-11,-10,-10,-9,
        -8,-7,-6,-5,-4,-3,-2,-1,0,1,1,2,3,4,5,6,6,7,8,9,9,10,11,11,12,12,13,14,14,15,15,
        15,16,16,16,17,17,17,17,18,18,18,18,18,18,18,18,18,18,18,18,18,17,17,17,17,16,16,
        16,16,15,15,14,14,14,13,13,12,12,11,11,11,10,10,9,9,8,8,7,7,6,6,5,5,4,4,3,3,2,2,1,
        1,0,0,-1,-1,-1,-2,-2,-3,-3,-3,-4,-4,-4,-4,-5,-5,-5,-5,-6,-6,-6,-6,-6,-6,-6,-7,-7,
        -7,-7,-7,-7,-7,-7,-7,-7,-7,-7,-7,-6,-6,-6,-6,-6,-6,-6,-6,-5,-5,-5,-5,-5,-4,-4,-4,
        -4,-4,-3,-3,-3,-3,-2,-2,-2,-2,-1,-1,-1,-1,-1,0,0,0,0,1,1,1,1,1,2,2,2,2,2,2,3,3,3,
        3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,3,3,3,3,3,3,
        3,3,3,3,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,-1,-1,-1,-1,-1,
        -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
        -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    ],
    [
        131072,131072,131072,131072,131072,131072,131072,131072,131072,131072,131072,
        131072,131072,131072,131072,131072,131072,131072,131072,131072,131072,131072,131072,
        131072,131072,131072,131072,131072,131072,131072,131072,131072,131071,131071,131071,
        131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,
        131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,
        131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,131071,
        131071,131071,131071,131071,131071,131071,131070,131070,131070,131070,131070,131070,
        131070,131070,131070,131070,131070,131070,131070,131070,131070,131070,131069,131069,
        131069,131069,131069,131069,131069,131069,131068,131068,131068,131068,131068,131068,
        131067,131067,131067,131067,131066,131066,131066,131066,131065,131065,131064,131064,
        131064,131063,131063,131062,131062,131061,131061,131060,131060,131059,131058,131058,
        131057,131056,131055,131055,131054,131053,131052,131051,131050,131049,131047,131046,
        131045,131044,131042,131041,131039,131038,131036,131034,131032,131031,131029,131027,
        131024,131022,131020,131017,131015,131012,131009,131007,131004,131001,130997,130994,
        130991,130987,130983,130979,130975,130971,130967,130962,130958,130953,130948,130942,
        130937,130931,130926,130920,130913,130907,130900,130893,130886,130879,130871,130863,
        130855,130847,130838,130829,130820,130810,130800,130790,130779,130768,130757,130745,
        130733,130721,130708,130695,130682,130668,130654,130639,130624,130608,130592,130576,
        130559,130541,130523,130505,130486,130466,130446,130426,130405,130383,130361,130338,
        130314,130290,130265,130240,130214,130187,130160,130132,130103,130074,130044,130013,
        129981,129949,129916,129882,129847,129812,129775,129738,129700,129661,129621,129581,
        129539,129497,129453,129409,129364,129318,129270,129222,129173,129123,129071,129019,
        128966,128911,128856,128799,128742,128683,128623,128562,128500,128436,128372,128306,
        128239,128171,128101,128031,127959,127886,127811,127736,127659,127580,127501,127420,
        127337,127254,127169,127082,126995,126906,126815,126723,126630,126535,126439,126341,
        126242,126142,126040,125936,125831,125725,125617,125507,125396,125284,125170,125055,
        124938,124819,124699,124577,124454,124329,124203,124075,123946,123815,123683,123549,
        123413,123276,123137,122997,122855,122711,122566,122420,122272,122122,121971,121818,
        121663,121507,121350,121191,121030,120868,120704,120539,120372,120204,120034,119863,
        119690,119515,119339,119162,118983,118803,118621,118438,118253,118067,117879,117690,
        117500,117308,117114,116920,116724,116526,116327,116127,115926,115723,115519,115313,
        115106,114898,114689,114478,114266,114053,113839,113624,113407,113189,112970,112750,
        112528,112306,112082,111858,111632,111405,111177,110948,110718,110487,110255,110022,
        109788,109553,109317,109080,108842,108604,108364,108124,107883,107641,107398,107154,
        106909,106664,106418,106171,105923,105675,105426,105176,104926,104675,104423,104171,
        103918,103664,103410,103155,102899,102643,102387,102130,101872,101614,101356,101096,
        100837,100577,100316,100055,99794,99532,99270,99007,98745,98481,98217,97953,97689,
        97424,97159,96894,96628,96362,96096,95829,95563,95296,95028,94761,94493,94225,93957,
        93688,93419,93151,92881,92612,92343,92073,91803,91534,91263,90993,90723,90452,90182,
        89911,89640,89369,89098,88827,88556,88284,88013,87741,87470,87198,86926,86654,86382,
        86110,85838,85566,85294,85022,84750,84477,84205,83933,83660,83388,83116,82843,82571,
        82298,82026,81753,81481,81208,80936,80663,80391,80118,79846,79573,79301,79029,78756,
        78484,78212,77939,77667,77395,77123,76851,76579,76307,76035,75763,75491,75220,74948,
        74676,74405,74134,73862,73591,73320,73049,72778,72507,72237,71966,71696,71426,71156,
        70886,70616,70346,70077,69807,69538,69269,69000,68732,68463,68195,67927,67659,67391,
        67124,66857,66590,66323,66056,65790,65524,65258,64993,64728,64463,64198,63934,63670,
        63406,63143,62880,62617,62354,62092,61830,61569,61308,61047,60787,60527,60267,60008,
        59749,59491,59233,58975,58718,58461,58205,57949,57694,57439,57184,56930,56676,56423,
        56171,55918,55667,55416,55165,54915,54665,54416,54167,53919,53671,53424,53178,52932,
        52686,52442,52197,51954,51710,51468,51226,50984,50743,50503,50263,50024,49786,49548,
        49310,49074,48838,48602,48367,48133,47899,47666,47434,47202,46971,46740,46510,46281,
        46052,45824,45597,45370,45144,44918,44693,44469,44245,44022,43800,43578,43357,43137,
        42917,42698,42479,42261,42044,41827,41612,41396,41181,40967,40754,40541,40329,40118,
        39907,39696,39487,39278,39069,38862,38655,38448,38242,38037,37832,37628,37425,37222,
        37020,36819,36618,36417,36218,36019,35820,35622,35425,35228,35032,34837,34642,34448,
        34254,34061,33869,33677,33486,33295,33105,32916,32727,32539,32352,32164,31978,31792,
        31607,31422,31238,31055,30872,30690,30508,30327,30147,29967,29787,29609,29431,29253,
        29076,28900,28724,28549,28374,28200,28027,27854,27682,27510,27339,27169,26999,26830,
        26661,26493,26326,26159,25993,25827,25662,25498,25334,25171,25008,24846,24685,24524,
        24364,24204,24045,23887,23729,23572,23416,23260,23104,22950,22796,22642,22490,22337,
        22186,22035,21884,21735,21586,21437,21289,21142,20996,20850,20704,20560,20416,20272,
        20129,19987,19845,19705,19564,19425,19285,19147,19009,18872,18735,18599,18464,18329,
        18195,18062,17929,17797,17665,17534,17404,17274,17145,17017,16889,16762,16635,16509,
        16384,16259,16135,16011,15888,15766,15644,15523,15403,15283,15163,15044,14926,14809,
        14692,14575,14460,14344,14230,14116,14002,13889,13777,13665,13554,13443,13333,13224,
        13115,13007,12899,12792,12685,12579,12473,12368,12264,12160,12056,11953,11851,11749,
        11648,11547,11447,11347,11248,11149,11051,10953,10856,10759,10663,10568,10472,10378,
        10284,10190,10097,10004,9912,9820,9729,9638,9548,9458,9369,9280,9191,9104,9016,8929,
        8843,8757,8671,8586,8501,8417,8333,8250,8167,8085,8003,7921,7840,7759,7679,7599,
        7520,7441,7363,7285,7207,7130,7053,6977,6901,6826,6751,6676,6602,6528,6455,6382,
        6310,6237,6166,6095,6024,5954,5884,5814,5745,5676,5608,5540,5473,5405,5339,5273,
        5207,5141,5076,5012,4947,4884,4820,4757,4694,4632,4570,4509,4448,4387,4327,4267,
        4208,4149,4090,4032,3974,3917,3859,3803,3746,3691,3635,3580,3525,3471,3417,3363,
        3310,3257,3204,3152,3100,3049,2998,2947,2897,2847,2797,2748,2699,2651,2603,2555,
        2507,2460,2414,2367,2321,2276,2230,2185,2141,2096,2052,2009,1966,1923,1880,1838,
        1796,1754,1713,1672,1631,1591,1551,1511,1472,1432,1394,1355,1317,1279,1242,1204,
        1167,1131,1094,1058,1023,987,952,917,882,848,814,780,747,713,680,648,615,583,551,
        519,488,457,426,396,365,335,305,276,246,217,188,160,132,103,76,48,21,-7,-34,-60,
        -87,-113,-139,-165,-190,-216,-241,-266,-290,-315,-339,-363,-387,-410,-434,-457,
        -480,-502,-525,-547,-569,-591,-613,-634,-656,-677,-697,-718,-739,-759,-779,-799,
        -818,-838,-857,-876,-895,-914,-932,-951,-969,-987,-1005,-1022,-1040,-1057,-1074,
        -1091,-1107,-1124,-1140,-1156,-1172,-1188,-1203,-1219,-1234,-1249,-1264,-1279,-1293,
        -1308,-1322,-1336,-1350,-1363,-1377,-1390,-1403,-1416,-1429,-1442,-1454,-1467,-1479,
        -1491,-1503,-1514,-1526,-1537,-1548,-1559,-1570,-1581,-1592,-1602,-1613,-1623,-1633,
        -1643,-1652,-1662,-1671,-1680,-1690,-1699,-1707,-1716,-1725,-1733,-1741,-1750,-1758,
        -1765,-1773,-1781,-1788,-1796,-1803,-1810,-1817,-1824,-1830,-1837,-1843,-1850,-1856,
        -1862,-1868,-1874,-1880,-1885,-1891,-1896,-1902,-1907,-1912,-1917,-1922,-1926,-1931,
        -1935,-1940,-1944,-1948,-1952,-1956,-1960,-1964,-1968,-1971,-1975,-1978,-1982,-1985,
        -1988,-1991,-1994,-1997,-2000,-2002,-2005,-2007,-2010,-2012,-2014,-2017,-2019,-2021,
        -2022,-2024,-2026,-2028,-2029,-2031,-2032,-2034,-2035,-2036,-2037,-2038,-2039,-2040,
        -2041,-2042,-2042,-2043,-2044,-2044,-2045,-2045,-2045,-2045,-2046,-2046,-2046,-2046,
        -2046,-2045,-2045,-2045,-2044,-2044,-2044,-2043,-2042,-2042,-2041,-2040,-2039,-2039,
        -2038,-2037,-2035,-2034,-2033,-2032,-2031,-2029,-2028,-2026,-2025,-2023,-2022,-2020,
        -2018,-2017,-2015,-2013,-2011,-2009,-2007,-2005,-2003,-2001,-1999,-1996,-1994,-1992,
        -1989,-1987,-1984,-1982,-1979,-1977,-1974,-1971,-1969,-1966,-1963,-1960,-1957,-1954,
        -1951,-1948,-1945,-1942,-1939,-1936,-1933,-1929,-1926,-1923,-1919,-1916,-1913,-1909,
        -1906,-1902,-1899,-1895,-1891,-1888,-1884,-1880,-1877,-1873,-1869,-1865,-1861,-1857,
        -1853,-1849,-1845,-1841,-1837,-1833,-1829,-1825,-1821,-1817,-1813,-1809,-1804,-1800,
        -1796,-1791,-1787,-1783,-1778,-1774,-1770,-1765,-1761,-1756,-1752,-1747,-1743,-1738,
        -1734,-1729,-1725,-1720,-1716,-1711,-1706,-1702,-1697,-1692,-1688,-1683,-1678,-1673,
        -1669,-1664,-1659,-1654,-1650,-1645,-1640,-1635,-1630,-1626,-1621,-1616,-1611,-1606,
        -1601,-1596,-1591,-1586,-1582,-1577,-1572,-1567,-1562,-1557,-1552,-1547,-1542,-1537,
        -1532,-1527,-1522,-1517,-1512,-1507,-1502,-1497,-1492,-1487,-1482,-1477,-1472,-1467,
        -1462,-1457,-1452,-1447,-1442,-1437,-1432,-1427,-1422,-1417,-1412,-1407,-1402,-1397,
        -1392,-1386,-1381,-1376,-1371,-1366,-1361,-1356,-1351,-1346,-1341,-1336,-1331,-1326,
        -1321,-1316,-1311,-1306,-1301,-1296,-1291,-1286,-1281,-1276,-1271,-1266,-1261,-1256,
        -1251,-1246,-1241,-1236,-1231,-1226,-1221,-1216,-1211,-1206,-1201,-1196,-1191,-1186,
        -1181,-1176,-1171,-1166,-1161,-1156,-1152,-1147,-1142,-1137,-1132,-1127,-1122,-1117,
        -1112,-1108,-1103,-1098,-1093,-1088,-1083,-1079,-1074,-1069,-1064,-1060,-1055,-1050,
        -1045,-1040,-1036,-1031,-1026,-1022,-1017,-1012,-1008,-1003,-998,-994,-989,-984,-980,
        -975,-970,-966,-961,-957,-952,-947,-943,-938,-934,-929,-925,-920,-916,-911,-907,
        -902,-898,-894,-889,-885,-880,-876,-872,-867,-863,-859,-854,-850,-846,-841,-837,
        -833,-828,-824,-820,-816,-812,-807,-803,-799,-795,-791,-787,-782,-778,-774,-770,
        -766,-762,-758,-754,-750,-746,-742,-738,-734,-730,-726,-722,-718,-714,-710,-706,
        -702,-699,-695,-691,-687,-683,-679,-676,-672,-668,-664,-661,-657,-653,-649,-646,
        -642,-638,-635,-631,-627,-624,-620,-617,-613,-609,-606,-602,-599,-595,-592,-588,
        -585,-581,-578,-574,-571,-568,-564,-561,-557,-554,-551,-547,-544,-541,-537,-534,
        -531,-528,-524,-521,-518,-515,-511,-508,-505,-502,-499,-496,-492,-489,-486,-483,
        -480,-477,-474,-471,-468,-465,-462,-459,-456,-453,-450,-447,-444,-441,-438,-435,
        -433,-430,-427,-424,-421,-418,-416,-413,-410,-407,-405,-402,-399,-396,-394,-391,
        -388,-386,-383,-380,-378,-375,-373,-370,-367,-365,-362,-360,-357,-355,-352,-350,
        -347,-345,-342,-340,-337,-335,-333,-330,-328,-325,-323,-321,-318,-316,-314,-311,
        -309,-307,-305,-302,-300,-298,-296,-293,-291,-289,-287,-285,-282,-280,-278,-276,
        -274,-272,-270,-268,-266,-264,-261,-259,-257,-255,-253,-251,-249,-247,-246,-244,
        -242,-240,-238,-236,-234,-232,-230,-228,-226,-225,-223,-221,-219,-217,-216,-214,
        -212,-210,-209,-207,-205,-203,-202,-200,-198,-197,-195,-193,-192,-190,-188,-187,
        -185,-183,-182,-180,-179,-177,-176,-174,-172,-171,-169,-168,-166,-165,-163,-162,
        -161,-159,-158,-156,-155,-153,-152,-151,-149,-148,-146,-145,-144,-142,-141,-140,
        -138,-137,-136,-134,-133,-132,-131,-129,-128,-127,-126,-124,-123,-122,-121,-120,
        -118,-117,-116,-115,-114,-113,-111,-110,-109,-108,-107,-106,-105,-104,-103,-102,
        -100,-99,-98,-97,-96,-95,-94,-93,-92,-91,-90,-89,-88,-87,-86,-85,-85,-84,-83,-82,
        -81,-80,-79,-78,-77,-76,-75,-75,-74,-73,-72,-71,-70,-70,-69,-68,-67,-66,-66,-65,
        -64,-63,-62,-62,-61,-60,-59,-59,-58,-57,-56,-56,-55,-54,-54,-53,-52,-52,-51,-50,
        -50,-49,-48,-48,-47,-46,-46,-45,-45,-44,-43,-43,-42,-42,-41,-40,-40,-39,-39,-38,
        -38,-37,-36,-36,-35,-35,-34,-34,-33,-33,-32,-32,-31,-31,-30,-30,-29,-29,-29,-28,
        -28,-27,-27,-26,-26,-25,-25,-25,-24,-24,-23,-23,-23,-22,-22,-21,-21,-21,-20,-20,
        -20,-19,-19,-18,-18,-18,-17,-17,-17,-16,-16,-16,-15,-15,-15,-15,-14,-14,-14,-13,
        -13,-13,-13,-12,-12,-12,-11,-11,-11,-11,-10,-10,-10,-10,-9,-9,-9,-9,-9,-8,-8,-8,-8,
        -7,-7,-7,-7,-7,-6,-6,-6,-6,-6,-6,-5,-5,-5,-5,-5,-5,-4,-4,-4,-4,-4,-4,-4,-3,-3,-3,
        -3,-3,-3,-3,-3,-2,-2,-2,-2,-2,-2,-2,-2,-2,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
        0,0,0,0,0,0,0,0,0,
    ]
];
