//! Port of microsim/devices/MSDevice_ToC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::to_string::f64_to_string_prec;
use crate::utils::vehicle::sumo_route_handler::SUMORouteHandler;

// shortcut: MSGlobals::gUseMesoSim is not ported yet (ms_globals.rs is empty), returns the C++ default; upgrade once MSGlobals is filled
fn g_use_meso_sim() -> bool {
    false
}


/// C++ enum `MSDevice_ToC::ToCState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_ToC_ToCState {
    UNDEFINED = 0,
    MANUAL = 1,
    AUTOMATED = 2,
    PREPARING_TOC = 3,
    MRM = 4,
    RECOVERING = 5,
}

/// The ToC Device controls transition of control between automated and manual driving.
/// C++ class `MSDevice_ToC` (microsim/devices/MSDevice_ToC.h)
#[repr(C)]
pub struct MSDevice_ToC {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// vehicle type ID for manual driving
    pub myManualTypeID: String,
    /// vehicle type ID for automated driving
    pub myAutomatedTypeID: String,
    /// Average response time needed by the driver to take back control
    pub myResponseTime: SUMOTime,
    /// Recovery rate for the driver's awareness after a ToC
    pub myRecoveryRate: f64,
    /// Level of the awareness below which no lane-changes are performed
    pub myLCAbstinence: f64,
    /// Average awareness the driver has initially after a ToC
    pub myInitialAwareness: f64,
    /// Deceleration rate applied during MRM
    pub myMRMDecel: f64,
    /// Current awareness-level of the driver in [0,1]
    pub myCurrentAwareness: f64,
    /// Coloring scheme,
    pub myColorScheme: BTreeMap<MSDevice_ToC_ToCState, RGBColor>,
    /// Whether a coloring scheme shall by applied to indicate the different toc stages,
    pub myUseColorScheme: bool,
    /// Current state of the device
    pub myState: MSDevice_ToC_ToCState,
    /// The holder vehicle casted to MSVehicle*
    pub myHolderMS: Ptr<MSVehicle>,
    pub myTriggerMRMCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    pub myTriggerToCCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    pub myRecoverAwarenessCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    pub myExecuteMRMCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    pub myPrepareToCCommand: Ptr<Unmapped /* WrappingCommand<MSDevice_ToC> */>,
    /// The file the devices output goes to
    pub myOutputFile: Ptr<OutputDevice>,
    /// Storage for events to be written to the output
    pub myEvents: VecDeque<(SUMOTime, String)>,
    /// Storage for events to be written to the output
    pub myEventLanes: VecDeque<(String, f64)>,
    /// Storage for events to be written to the output
    pub myEventXY: VecDeque<(f64, f64)>,
    /// LC mode overridden during MRM, stored for restoration
    pub myPreviousLCMode: i32,
    /// Parameters for the openGap mechanism applied during ToC preparation phase
    pub myOpenGapParams: MSDevice_ToC_OpenGapParams,
    /// Duration in s. for which the vehicle needs to be able to follow its route without a lane change to continue in automated mode (only has effect if dynamic ToCs are activated,
    pub myDynamicToCThreshold: f64,
    /// Probability of an MRM to occur after a dynamically triggered ToC
    pub myMRMProbability: f64,
    /// Switch for considering dynamic ToCs,
    pub myDynamicToCActive: bool,
    /// Flag to indicate that a dynamically triggered ToC is in preparation
    pub myIssuedDynamicToC: bool,
    /// Lane, on which the ongoing dynamic ToC was issued. It can only be aborted if the lane was changed.
    pub myDynamicToCLane: i32,
    /// Whether vehicle tries to change to the right during an MRM
    pub myMRMKeepRight: bool,
    /// stop vehicle tries to reach during MRM
    pub myMRMSafeSpot: String,
    /// duration at stop vehicle tries to reach during MRM
    pub myMRMSafeSpotDuration: SUMOTime,
    /// Maximal acceleration that may be applied during the ToC preparation phase TODO: Make effective
    pub myMaxPreparationAccel: f64,
    /// Storage for original maximal acceleration of vehicle.
    pub myOriginalMaxAccel: f64,
}

impl std::ops::Deref for MSDevice_ToC {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_ToC {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_ToC (port as module-level statics, see AGENTS.md):
//   static int MSDevice_ToC::myInstances;
//   static int MSDevice_ToC::createdOutputFiles;
//   static int MSDevice_ToC::LCModeMRM;
//   static int MSDevice_ToC::lookupResponseTimeMRMProbs;
//   static int MSDevice_ToC::lookupResponseTimeLeadTimes;
//   static int MSDevice_ToC::lookupResponseTimeVariances;
//   static SumoRNG MSDevice_ToC::myResponseTimeRNG;


// C++ #defines of MSDevice_ToC.cpp
const DEFAULT_RESPONSE_TIME: f64 = -1.0;
const DEFAULT_RECOVERY_RATE: f64 = 0.1;
const DEFAULT_LCABSTINENCE: f64 = 0.0;
const DEFAULT_INITIAL_AWARENESS: f64 = 0.5;
const DEFAULT_MRM_DECEL: f64 = 1.5;
const DEFAULT_DYNAMIC_TOC_THRESHOLD: f64 = 0.0;
const DEFAULT_MRM_PROBABILITY: f64 = 0.05;
const DYNAMIC_TOC_LEADTIME_FACTOR: f64 = 0.75;
const DYNAMIC_TOC_ABORT_RESISTANCE_FACTOR: f64 = 2.0;
const DEFAULT_OPENGAP_TIMEGAP: f64 = -1.0;
const DEFAULT_OPENGAP_SPACING: f64 = 0.0;
const DEFAULT_OPENGAP_CHANGERATE: f64 = 1.0;
const DEFAULT_OPENGAP_MAXDECEL: f64 = 1.0;
const MAX_RESPONSETIME_SAMPLE_TRIES: i32 = 100;
const MAX_RESPONSETIME_VARIANCE: f64 = 10000.0;
// SPEED_EPS from config.h.cmake
const SPEED_EPS: f64 = 0.01;
// C++ `static int MSDevice_ToC::LCModeMRM = 768`
const LCModeMRM: i32 = 768;
// C++ `VEHPARS_COLOR_SET` (SUMOVehicleParameter.h)
const VEHPARS_COLOR_SET: i64 = 1;

thread_local! {
    /// MSDevice_ToC::myInstances (registered devices)
    static MY_INSTANCES: std::cell::RefCell<BTreeSet<Ptr<MSDevice_ToC>>> = std::cell::RefCell::new(BTreeSet::new());
    /// MSDevice_ToC::createdOutputFiles
    static CREATED_OUTPUT_FILES: std::cell::RefCell<BTreeSet<String>> = std::cell::RefCell::new(BTreeSet::new());
    /// MSDevice_ToC::myResponseTimeRNG, seeded as in C++ (`SumoRNG MSDevice_ToC::myResponseTimeRNG("toc")`)
    static MY_RESPONSE_TIME_RNG: std::cell::RefCell<SumoRNG> = std::cell::RefCell::new(SumoRNG::new("toc"));
}

// Grid of the response time distribution (MSDevice_ToC.cpp, generated by generateResponseTimeDistributions.py).
// Probability for an MRM to occur
const LOOKUP_RESPONSE_TIME_MRM_PROBS: &[f64] = &[0.0, 0.05, 0.1, 0.15000000000000002, 0.2, 0.25, 0.30000000000000004, 0.35000000000000003, 0.4, 0.45, 0.5];
// Lead time grid
const LOOKUP_RESPONSE_TIME_LEAD_TIMES: &[f64] = &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.7999999999999999, 0.8999999999999999, 0.9999999999999999, 1.0999999999999999, 1.2, 1.3, 1.4, 1.5, 1.5999999999999999, 1.7, 1.8, 1.9, 2.0, 2.25, 2.5, 2.75, 3.0, 3.25, 3.5, 3.75, 4.0, 4.25, 4.5, 4.75, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5, 9.0, 9.5, 10.0, 10.5, 11.0, 11.5, 12.0, 12.5, 13.0, 13.5, 14.0, 14.5, 15.0, 15.5, 16.0, 16.5, 17.0, 17.5, 18.0, 18.5, 19.0, 19.5, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0, 47.0, 48.0, 49.0, 50.0];
// Variances of the response time distribution.
const LOOKUP_RESPONSE_TIME_VARIANCES: &[&[f64]] = &[
    &[0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001, 0.0001],
    &[0.018238371642696278, 0.03647674328603705, 0.054715114926535656, 0.07295348656987645, 0.09119185821321724, 0.10943022985371582, 0.12766860149705656, 0.14590697314039733, 0.16414534478089599, 0.18238371642423673, 0.2006220880675775, 0.21886045971091828, 0.2370988313514169, 0.25533720299475765, 0.27357557463809845, 0.291813946278597, 0.3100523179219377, 0.32829068956527846, 0.3465290612057772, 0.36476743284911795, 0.4103633619560487, 0.45595929106297967, 0.5015552201670682, 0.5471511492739992, 0.59274707838093, 0.6383430074850186, 0.6839389365919495, 0.7295348656988803, 0.7751307948058112, 0.8207267239098999, 0.8663226530168309, 0.9119185821237615, 1.003110440334781, 1.0943022985486428, 1.1854941567596624, 1.2766860149735242, 1.3678778731845436, 1.4590697313984053, 1.622588042723657, 1.8236013818166044, 2.0271872430355344, 2.2329896377033402, 2.440681612959606, 2.6499766330096066, 2.8606328744048484, 3.0724517481497657, 3.285273075453899, 3.4989689745182173, 3.713437758931686, 3.928598499444084, 4.144386478335934, 4.360749535794346, 4.577645204319768, 4.795038495182951, 5.012900204026633, 5.231205620052272, 5.449933544477286, 5.669065543877604, 5.888585381094657, 6.108478580034079, 6.328732091514834, 6.5493340353396325, 6.991540386888257, 7.4350193801571836, 7.8797023900653835, 8.325528107903486, 8.772441019472117, 9.220390365425358, 9.66932940241786, 10.11921485679467, 10.570006505095746, 11.021666840703753, 11.474160800924851, 11.927455537955435, 12.381520222795276, 12.836325874663427, 13.291845210806684, 13.748052512926236, 14.204923507573522, 14.662435258383752, 15.120566068535554, 15.57929539219183, 16.03860375377652, 16.498472674274336, 16.958884603774735, 17.41982285960362, 17.881271569514066, 18.343215619413176, 18.805640605235663, 19.268532788517863, 19.731879055399546, 20.195666878723525],
    &[0.023394708584543455, 0.04678941716973141, 0.07018412575207719, 0.09357883433726513, 0.11697354292245306, 0.14036825150764104, 0.16376296009282898, 0.18715766867517475, 0.2105523772603627, 0.23394708584555063, 0.25734179443073857, 0.2807365030130844, 0.3041312115982723, 0.3275259201834602, 0.3509206287686481, 0.37431533735099387, 0.3977100459361818, 0.42110475452136986, 0.4444994631065578, 0.46789417168890357, 0.5263809431504523, 0.5848677146120012, 0.6433544860735498, 0.7018412575350985, 0.7603280289966473, 0.8188148004581961, 0.8773015719197449, 0.9357883433784517, 0.9942751148400004, 1.0527618863015489, 1.111248657763098, 1.169735429224647, 1.2867089721477445, 1.4036825150708419, 1.520656057991097, 1.6376296009141948, 1.7546031438372918, 1.8715766867603902, 2.080324923070349, 2.3356056603653466, 2.592695753086048, 2.851065033244823, 3.110348235805368, 3.37030701064756, 3.630792243424829, 3.8917140872434826, 4.153020535352749, 4.414682984212653, 4.676686852454919, 4.939025633083471, 5.201697195938797, 5.4647015339779985, 5.728039423002128, 5.991711652753062, 6.255718611539178, 6.520060086487768, 6.784735192689795, 7.0497423770254475, 7.315079462951026, 7.580743715621537, 7.846731914823575, 8.113040428416943, 8.646602223970579, 9.181394311584102, 9.717379133745458, 10.25451769188949, 10.792770521511661, 11.332098388641393, 11.872462773672844, 12.41382619675905, 12.956152426938662, 13.499406606935544, 14.04355531739462, 14.588566598200957, 15.134409939740127, 15.681056253544272, 16.228477829207687, 16.776648282531472, 17.32554249848686, 17.875136571609353, 18.425407745596857, 18.976334353419617, 19.527895758814104, 20.080072299738145, 20.632845234156534, 21.18619668838177, 21.740109608063758, 22.294567711842525, 22.849555447666962, 23.405057951674344, 23.96106100953682, 24.517551020220964],
    &[0.028809965676139145, 0.05761993135292278, 0.08642989702686427, 0.11523986270364789, 0.14404982838043154, 0.172859794054373, 0.20166975973115658, 0.23047972540794023, 0.2592896910818817, 0.2880996567586654, 0.31690962243544896, 0.34571958811223263, 0.37452955378617403, 0.4033395194629576, 0.4321494851397413, 0.4609594508136828, 0.4897694164904665, 0.51857938216725, 0.5473893478411916, 0.5761993135179752, 0.6482242277085132, 0.7202491418990512, 0.7922740560867471, 0.864298970277285, 0.9363238844678232, 1.0083487986555189, 1.0803737128460569, 1.152398627036595, 1.224423541227133, 1.296448455414829, 1.368473369605367, 1.4404982837959048, 1.584548112174139, 1.7285979405552145, 1.8726477689334486, 2.0166975973145242, 2.1607474256927586, 2.3047972540738346, 2.5572219533483644, 2.8623099438447785, 3.167154753839855, 3.4715923078311928, 3.775618300633292, 4.079301412505033, 4.3827364323759, 4.686020985499979, 4.989245061612653, 5.2924871133699085, 5.595813357348367, 5.89927852360174, 6.202927167199209, 6.506795109670751, 6.810910812533207, 7.115296603265157, 7.419969732096102, 7.724943265058281, 8.030226829842663, 8.33582723446787, 8.641748978492437, 8.947994674556652, 9.254565395777925, 9.561460961999748, 10.17622101728854, 10.792256328037844, 11.409539737303032, 12.02803871883203, 12.647717622463388, 13.268539213916304, 13.890465727714453, 14.513459582113466, 15.137483858085837, 15.76250261298258, 16.388481078046894, 17.015385774331957, 17.643184571383188, 18.271846706039163, 18.901342773672106, 19.531644700723955, 20.16272570482796, 20.79456024708863, 21.427123979740387, 22.06039369148417, 22.694347252144688, 23.32896355779389, 23.964222477085105, 24.600104799357926, 25.23659218482918, 25.873667117046345, 26.51131285772261, 27.149513403967962, 27.788253447896256, 28.427518338543063],
    &[0.03496845765860337, 0.06993691531785123, 0.1049053729770991, 0.13987383063634692, 0.17484228829559478, 0.20981074595200053, 0.24477920361124836, 0.2797476612704962, 0.314716118929744, 0.34968457658899194, 0.38465303424823977, 0.41962149190748765, 0.4545899495638933, 0.48955840722314126, 0.5245268648823892, 0.5594953225416369, 0.5944637802008848, 0.6294322378601326, 0.6644006955193805, 0.6993691531757862, 0.7867902973239058, 0.8742114414720256, 0.961632585617303, 1.0490537297654225, 1.1364748739135424, 1.2238960180588196, 1.3113171622069395, 1.3987383063550594, 1.4861594505003368, 1.5735805946484562, 1.6610017387965759, 1.748422882941853, 1.9232651712352506, 2.0981074595314895, 2.272949747824887, 2.4477920361182846, 2.6226343244145234, 2.7974766127079205, 3.0926745755509586, 3.44395841916428, 3.7929654946149927, 4.140199155580832, 4.48610487276643, 4.8310499148399675, 5.175327720605737, 5.519169410160118, 5.862755881539858, 6.20622837304298, 6.549697059107542, 6.893247832651301, 7.236947584569466, 7.580848293751031, 7.924990195098607, 8.2694042393908, 8.614114010475241, 8.959137226336916, 9.30448692001335, 9.650172373181427, 9.996199857775714, 10.342573227835842, 10.689294393844476, 11.036363704475946, 11.73154213703699, 12.428090426873897, 13.125980076789245, 13.825176212794954, 14.52564028426291, 15.227331883606062, 15.930209972250445, 16.634233702118422, 17.339362959284276, 18.045558715641334, 18.752783247333983, 19.461000260660008, 20.170174953762714, 20.880274034013603, 21.591265705145904, 22.303119634055196, 23.015806904341318, 23.7292999616137, 24.443572554090043, 25.158599671023588, 25.874357480669964, 26.590823269047004, 27.30797538027189, 28.025793159035764, 28.74425689552527, 29.46334777297606, 30.183047817969005, 30.903339853423454, 31.624207454305022, 32.345634905899644],
    &[0.04208452197242317, 0.08416904394549082, 0.12625356591855852, 0.16833808789162616, 0.21042260986469383, 0.25250713183776147, 0.294591653807987, 0.33667617578105463, 0.3787606977541223, 0.42084521972719, 0.46292974170025764, 0.5050142636733252, 0.5470987856463931, 0.5891833076194606, 0.6312678295925284, 0.6733523515627537, 0.7154368735358214, 0.7575213955088891, 0.7996059174819568, 0.8416904394550245, 0.9469017443876938, 1.0521130493203628, 1.1573243542501899, 1.2625356591828591, 1.3677469641155282, 1.472958269045355, 1.5781695739780248, 1.6833808789106937, 1.7885921838433625, 1.8938034887731896, 1.9990147937058587, 2.104226098638527, 2.314648708501024, 2.5250713183663627, 2.7354939282288586, 2.9459165380941967, 3.1563391479566936, 3.3667617578220317, 3.703839316684887, 4.098607599735211, 4.490071039515135, 4.879192386567447, 5.266673871161598, 5.6530296960510835, 6.038639278868162, 6.423784955842712, 6.808678484059117, 7.1934797498117895, 7.578310096881422, 7.9632619279700085, 8.348405702022065, 8.733795092290785, 9.119470830182808, 9.505463599150936, 9.891796234154242, 10.278485408041451, 10.665542934966393, 11.05297678524097, 11.440791880843797, 11.828990722803214, 12.217573888696098, 12.606540429161571, 13.385614043672865, 14.166184035906006, 14.948213302277296, 15.731659164963906, 16.51647601334669, 17.302617053034968, 18.09003546045466, 18.87868513730469, 19.66852119352658, 20.459500245021562, 21.251580584675224, 22.044722266798285, 22.838887132598156, 23.634038796019215, 24.430142603281702, 25.22716557558935, 26.025076341551085, 26.823845063912813, 27.62344336382784, 28.423844244828874, 29.22502201803638, 30.026952229552563, 30.82961159071451, 31.63297791152784, 32.43703003753926, 33.24174779014256, 34.047111910360435, 34.85310400598881, 35.65970650197814, 36.46690259392711],
    &[0.05029020480396514, 0.10058040960573261, 0.15087061441034225, 0.2011608192121097, 0.2514510240167194, 0.301741228821329, 0.35203143362309647, 0.40232163842770624, 0.45261184322947356, 0.5029020480340831, 0.5531922528358507, 0.6034824576404602, 0.6537726624450699, 0.7040628672468374, 0.7543530720514469, 0.8046432768532146, 0.8549334816578241, 0.9052236864595916, 0.9555138912642014, 1.0058040960688106, 1.1315296080760717, 1.2572551200833324, 1.382980632093436, 1.5087061441006968, 1.6344316561107992, 1.7601571681180603, 1.8858826801253215, 2.0116081921354243, 2.137333704142684, 2.263059216152788, 2.3887847281600485, 2.514510240170152, 2.765961264184674, 3.017412288202038, 3.2688633122194015, 3.520314336236765, 3.7717653602541295, 4.023216384271493, 4.402923686706604, 4.840923169412718, 5.275628140231794, 5.708106387011112, 6.139096012777341, 6.569117835239806, 6.998545742885471, 7.427651941064436, 7.856636809801444, 8.28564907733453, 8.714799743029685, 9.144171870687785, 9.573827594895418, 10.00381321001211, 10.43416291693517, 10.864901615202994, 11.296047006509264, 11.72761119486633, 12.159601914603718, 12.592023480161025, 13.024877525833228, 13.458163585558754, 13.891879549849177, 14.326022027631314, 15.195568219985953, 16.066758980781113, 16.93954397110994, 17.813869202173244, 18.68967930447209, 19.566918991345418, 20.44553399181503, 21.325471630737475, 22.206681173178218, 23.089114010781095, 23.972723742457497, 24.857466184805716, 25.743299336515662, 26.63018331333154, 27.518080264958197, 28.4069542817556, 29.296771296561822, 30.187498985305254, 31.07910666877645, 31.97156521721369, 32.86484695863455, 33.75892559154721, 34.653776102328635, 35.54937468734068, 36.44569867980954, 37.34272648128694, 38.24043749755077, 39.13881207875172, 40.03783146350794, 40.937477726773984],
    &[0.05974016468300759, 0.11948032936381749, 0.17922049404462742, 0.23896065872827943, 0.29870082340908927, 0.35844098808989927, 0.4181811527735513, 0.47792131745436117, 0.5376614821380133, 0.5974016468188231, 0.657141811499633, 0.7168819761832851, 0.7766221408640949, 0.8363623055449051, 0.8961024702285572, 0.955842634909367, 1.0155827995930191, 1.0753229642738293, 1.135063128954639, 1.1948032936382913, 1.344153705341737, 1.493504117048025, 1.6428545287514704, 1.7922049404549163, 1.9415553521612046, 2.0909057638646504, 2.2402561755709387, 2.389606587274385, 2.5389569989806726, 2.688307410684118, 2.8376578223904056, 2.987008234093852, 3.2857090575035857, 3.58440988091332, 3.8831107043230535, 4.181811527729945, 4.48051235113968, 4.779213174549414, 5.204876111923692, 5.688848562937568, 6.170159044583231, 6.6497495644421285, 7.128261753669921, 7.606142949678828, 8.083710709627423, 8.561193555361694, 9.038757543068261, 9.516524068724227, 9.99458208377549, 10.47299664830626, 10.951815027051936, 11.431071102373838, 11.910788612986488, 12.39098355983018, 12.871666012589971, 13.352841479046571, 13.834511951858257, 14.316676714667919, 14.799332966945224, 15.282476311040416, 15.766101133711286, 16.250200906162327, 17.219795978620663, 18.1911988298489, 19.164342959599296, 20.139160814458737, 21.115585503946967, 22.093551853025446, 23.07299702417972, 24.053860858945782, 25.036086036434696, 26.019618113131642, 27.004405486541614, 27.990399311257423, 28.977553386435417, 29.965824027565322, 30.95516993094154, 31.94555203660525, 32.93693339332716, 33.92927902800424, 34.92255582087156, 35.91673238727074, 36.91177896637648, 37.907667316880286, 38.90437061956949, 39.90186338652862, 40.90012137664726, 41.899121517093974, 42.89884183037187, 43.8992613666098, 44.90036014068208, 45.90211907383619],
    &[0.07067515415184052, 0.14135030830148337, 0.2120254624511261, 0.2827006166036112, 0.35337577075325394, 0.4240509249028968, 0.49472607905538185, 0.5654012332050246, 0.6360763873575095, 0.7067515415071527, 0.7774266956567951, 0.8481018498092803, 0.9187770039589231, 0.9894521581085659, 1.060127312261051, 1.1308024664106937, 1.2014776205603366, 1.2721527747128218, 1.3428279288624643, 1.41350308301495, 1.5901909683904774, 1.7668788537660056, 1.943566739144376, 2.120254624519904, 2.2969425098982743, 2.473630395273803, 2.6503182806493313, 2.8270061660277013, 3.0036940514032295, 3.1803819367816004, 3.3570698221571273, 3.5337577075354982, 3.8871334782865548, 4.240509249040453, 4.593885019794351, 4.947260790548251, 5.300636561302148, 5.654012332053204, 6.131630444292413, 6.6672022906208355, 7.200987739963464, 7.733716006955079, 8.265882445019285, 8.797830454475601, 9.32980159851235, 9.861967438834494, 10.394450401173598, 10.927337816806162, 11.460691587201293, 11.994554966496775, 12.528957402009762, 13.06391803975272, 13.599448295828148, 14.135553763816139, 14.672235643520898, 15.209491820318211, 15.747317686564726, 16.285706770680626, 16.82465122144653, 17.364142182462015, 17.904170082649248, 18.444724862108774, 19.52737339128757, 20.612003289072593, 21.69853012990063, 22.786871401229217, 23.87694757433194, 24.968682680039276, 26.062004572321317, 27.156844995848896, 28.253139532174295, 29.350827472729005, 30.449851649907924, 31.55015824661291, 32.651696597281045, 33.75441898881355, 34.85828046658357, 35.96323864863796, 37.06925354984671, 38.176287416815526, 39.2843045737864, 40.39327127936956, 41.50315559373177, 42.613927255723794, 43.72555756930443, 44.838019298684486, 45.951286571522466, 47.065334789581065, 48.18014054623036, 49.29568155028052, 50.41193655559852, 51.52888529603219],
    &[0.0834623671324666, 0.16692473426273555, 0.2503871013958466, 0.33384946852611547, 0.4173118356592266, 0.5007742027894957, 0.5842365699226064, 0.6676989370528754, 0.7511613041859866, 0.8346236713162553, 0.9180860384493664, 1.0015484055796355, 1.0850107727127467, 1.1684731398430155, 1.2519355069761264, 1.3353978741063957, 1.4188602412395066, 1.5023226083697758, 1.5857849755028866, 1.6692473426331556, 1.8779032604630912, 2.0865591782930264, 2.29521509612012, 2.5038710139500555, 2.7125269317799905, 2.921182849609927, 3.1298387674370196, 3.338494685266956, 3.547150603096892, 3.7558065209268268, 3.96446243875392, 4.173118356583855, 4.590430192243725, 5.007742027900754, 5.425053863557784, 5.842365699217655, 6.259677534874682, 6.676989370534554, 7.215284811865045, 7.810942636066625, 8.405713438558344, 9.000109193262631, 9.594476065866042, 10.189051324756685, 10.783998757795331, 11.379431470741277, 11.975426983124095, 12.572037465005534, 13.169296821117609, 13.767225679430393, 14.365834957075359, 14.965128442509108, 15.565104686191797, 16.165758398192054, 16.767081489665355, 17.369063854126264, 17.971693956723634, 18.574959280506526, 19.178846665295886, 19.78334256540095, 20.38843324553279, 20.994104929469565, 22.20713664504607, 23.42233029997001, 24.63958259994195, 25.858795291719396, 27.079875555554814, 28.302736078822026, 29.527294941445533, 30.75347539415742, 31.981205580215, 33.21041823195959, 34.44105036161415, 35.67304295805245, 36.90634069627536, 38.14089166337484, 39.3766471025938, 40.61356117597085, 41.851590745280404, 43.090695170440334, 44.33083612445286, 45.57197742365028, 46.814084872185866, 48.05712611949615, 49.301070529748834, 50.54588906213925, 51.79155416112571, 53.0380396556748, 54.285320666751495, 55.533373522224736, 56.782175678619716, 58.031705649011975],
    &[0.09864342769295685, 0.19728685538371601, 0.2959302830773173, 0.39457371076807657, 0.49321713846167803, 0.5918605661524371, 0.6905039938460386, 0.7891474215367978, 0.887790849230399, 0.9864342769211584, 1.0850777046147597, 1.1837211323055188, 1.2823645599991202, 1.38100798768988, 1.4796514153834808, 1.5782948430742403, 1.6769382707678415, 1.7755816984586006, 1.874225126152202, 1.972868553842961, 2.219477123074122, 2.4660856923052834, 2.7126942615364444, 2.9593028307647646, 3.2059113999959257, 3.4525199692270867, 3.6991285384582473, 3.945737107686567, 4.192345676917727, 4.438954246148888, 4.68556281538005, 4.932171384608369, 5.425388523070691, 5.918605661530172, 6.411822799992494, 6.905039938451975, 7.398257076914296, 7.891474215373778, 8.502335361374866, 9.169831074405188, 9.837265003168877, 10.504960425335845, 11.173131332846934, 11.841919360545138, 12.511417180930938, 13.181683762841976, 13.85275456878272, 14.524648510968875, 15.197372779403622, 15.870926242469201, 16.545301871890324, 17.220488489846183, 17.896472038316816, 18.573236507380663, 19.250764617377587, 19.929038321651447, 20.608039177427425, 21.287748619021162, 21.968148158299684, 22.64921953063529, 23.330944799863957, 24.013306432352234, 25.37987095153259, 26.74878238692951, 28.119918223260417, 29.4931641334551, 30.8684136985658, 32.245567980998565, 33.624535031317535, 35.005229375702534, 36.38757151118004, 37.77148742362557, 39.156908136182956, 40.54376929127801, 41.93201076678101, 43.32157632534552, 44.712413295099445, 46.1044722794653, 47.497706893719666, 48.892073525949804, 50.2875311201024, 51.68404097903562, 53.081566585544806, 54.48007343965984, 55.87952891050276, 57.27990210130182, 58.68116372625177, 60.08328599798667, 61.48624252468206, 62.890008215820075, 64.29455919573915, 65.6998727243063],
];

impl MSDevice_ToC {
    /// Records an event for the output (the C++ pushes onto myEvents, myEventLanes and myEventXY)
    fn record_event(&mut self, t: SUMOTime, name: &str) {
        self.myEvents.push_back((t, name.to_string()));
        let lane = self.myHolderMS.get_lane();
        let lanePos = self.myHolderMS.get_position_on_lane();
        self.myEventLanes.push_back((lane.get_id().to_string(), lanePos));
        let pos = self.myHolderMS.get_position(0.);
        self.myEventXY.push_back((pos.x(), pos.y()));
    }
}

impl MSDevice_ToC {
    /// C++: `void MSDevice_ToC::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_ToC-options
    // void
    // MSDevice_ToC::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("ToC Device");
    //     insertDefaultAssignmentOptions("toc", "ToC Device", oc);
    //
    //     oc.doRegister("device.toc.manualType", new Option_String());
    //     oc.addDescription("device.toc.manualType", "ToC Device", TL("Vehicle type for manual driving regime."));
    //     oc.doRegister("device.toc.automatedType", new Option_String());
    //     oc.addDescription("device.toc.automatedType", "ToC Device", TL("Vehicle type for automated driving regime."));
    //     oc.doRegister("device.toc.responseTime", new Option_Float(DEFAULT_RESPONSE_TIME));
    //     oc.addDescription("device.toc.responseTime", "ToC Device", TL("Average response time needed by a driver to take back control."));
    //     oc.doRegister("device.toc.recoveryRate", new Option_Float(DEFAULT_RECOVERY_RATE));
    //     oc.addDescription("device.toc.recoveryRate", "ToC Device", TL("Recovery rate for the driver's awareness after a ToC."));
    //     oc.doRegister("device.toc.lcAbstinence", new Option_Float(DEFAULT_LCABSTINENCE));
    //     oc.addDescription("device.toc.lcAbstinence", "ToC Device", TL("Attention level below which a driver restrains from performing lane changes (value in [0,1])."));
    //     oc.doRegister("device.toc.initialAwareness", new Option_Float(DEFAULT_INITIAL_AWARENESS));
    //     oc.addDescription("device.toc.initialAwareness", "ToC Device", TL("Average awareness a driver has initially after a ToC (value in [0,1])."));
    //     oc.doRegister("device.toc.mrmDecel", new Option_Float(DEFAULT_MRM_DECEL));
    //     oc.addDescription("device.toc.mrmDecel", "ToC Device", TL("Deceleration rate applied during a 'minimum risk maneuver'."));
    //     oc.doRegister("device.toc.dynamicToCThreshold", new Option_Float(DEFAULT_DYNAMIC_TOC_THRESHOLD));
    //     oc.addDescription("device.toc.dynamicToCThreshold", "ToC Device", TL("Time, which the vehicle requires to have ahead to continue in automated mode. The default value of 0 indicates no dynamic triggering of ToCs."));
    //     oc.doRegister("device.toc.dynamicMRMProbability", new Option_Float(DEFAULT_MRM_PROBABILITY));
    //     oc.addDescription("device.toc.dynamicMRMProbability", "ToC Device", TL("Probability that a dynamically triggered TOR is not answered in time."));
    //     oc.doRegister("device.toc.mrmKeepRight", new Option_Bool(false));
    //     oc.addDescription("device.toc.mrmKeepRight", "ToC Device", TL("If true, the vehicle tries to change to the right during an MRM."));
    //     oc.doRegister("device.toc.mrmSafeSpot", new Option_String());
    //     oc.addDescription("device.toc.mrmSafeSpot", "ToC Device", TL("If set, the vehicle tries to reach the given named stopping place during an MRM."));
    //     oc.doRegister("device.toc.mrmSafeSpotDuration", new Option_Float(60.));
    //     oc.addDescription("device.toc.mrmSafeSpotDuration", "ToC Device", TL("Duration the vehicle stays at the safe spot after an MRM."));
    //     oc.doRegister("device.toc.maxPreparationAccel", new Option_Float(0.0));
    //     oc.addDescription("device.toc.maxPreparationAccel", "ToC Device", TL("Maximal acceleration that may be applied during the ToC preparation phase."));
    //     oc.doRegister("device.toc.ogNewTimeHeadway", new Option_Float(-1.0));
    //     oc.addDescription("device.toc.ogNewTimeHeadway", "ToC Device", TL("Timegap for ToC preparation phase."));
    //     oc.doRegister("device.toc.ogNewSpaceHeadway", new Option_Float(-1.0));
    //     oc.addDescription("device.toc.ogNewSpaceHeadway", "ToC Device", TL("Additional spacing for ToC preparation phase."));
    //     oc.doRegister("device.toc.ogMaxDecel", new Option_Float(-1.0));
    //     oc.addDescription("device.toc.ogMaxDecel", "ToC Device", TL("Maximal deceleration applied for establishing increased gap in ToC preparation phase."));
    //     oc.doRegister("device.toc.ogChangeRate", new Option_Float(-1.0));
    //     oc.addDescription("device.toc.ogChangeRate", "ToC Device", TL("Rate of adaptation towards the increased headway during ToC preparation."));
    //     oc.doRegister("device.toc.useColorScheme", new Option_Bool(true));
    //     oc.addDescription("device.toc.useColorScheme", "ToC Device", TL("Whether a coloring scheme shall by applied to indicate the different ToC stages."));
    //     oc.doRegister("device.toc.file", new Option_String());
    //     oc.addDescription("device.toc.file", "ToC Device", TL("Switches on output by specifying an output filename."));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        oc.add_option_sub_topic("ToC Device");
        MSDevice::insert_default_assignment_options("toc", "ToC Device", oc, false);

        oc.do_register("device.toc.manualType", SumoOption::string());
        oc.add_description("device.toc.manualType", "ToC Device", "Vehicle type for manual driving regime.");
        oc.do_register("device.toc.automatedType", SumoOption::string());
        oc.add_description("device.toc.automatedType", "ToC Device", "Vehicle type for automated driving regime.");
        oc.do_register("device.toc.responseTime", SumoOption::float(DEFAULT_RESPONSE_TIME));
        oc.add_description("device.toc.responseTime", "ToC Device", "Average response time needed by a driver to take back control.");
        oc.do_register("device.toc.recoveryRate", SumoOption::float(DEFAULT_RECOVERY_RATE));
        oc.add_description("device.toc.recoveryRate", "ToC Device", "Recovery rate for the driver's awareness after a ToC.");
        oc.do_register("device.toc.lcAbstinence", SumoOption::float(DEFAULT_LCABSTINENCE));
        oc.add_description("device.toc.lcAbstinence", "ToC Device", "Attention level below which a driver restrains from performing lane changes (value in [0,1]).");
        oc.do_register("device.toc.initialAwareness", SumoOption::float(DEFAULT_INITIAL_AWARENESS));
        oc.add_description("device.toc.initialAwareness", "ToC Device", "Average awareness a driver has initially after a ToC (value in [0,1]).");
        oc.do_register("device.toc.mrmDecel", SumoOption::float(DEFAULT_MRM_DECEL));
        oc.add_description("device.toc.mrmDecel", "ToC Device", "Deceleration rate applied during a 'minimum risk maneuver'.");
        oc.do_register("device.toc.dynamicToCThreshold", SumoOption::float(DEFAULT_DYNAMIC_TOC_THRESHOLD));
        oc.add_description("device.toc.dynamicToCThreshold", "ToC Device", "Time, which the vehicle requires to have ahead to continue in automated mode. The default value of 0 indicates no dynamic triggering of ToCs.");
        oc.do_register("device.toc.dynamicMRMProbability", SumoOption::float(DEFAULT_MRM_PROBABILITY));
        oc.add_description("device.toc.dynamicMRMProbability", "ToC Device", "Probability that a dynamically triggered TOR is not answered in time.");
        oc.do_register("device.toc.mrmKeepRight", SumoOption::bool(false));
        oc.add_description("device.toc.mrmKeepRight", "ToC Device", "If true, the vehicle tries to change to the right during an MRM.");
        oc.do_register("device.toc.mrmSafeSpot", SumoOption::string());
        oc.add_description("device.toc.mrmSafeSpot", "ToC Device", "If set, the vehicle tries to reach the given named stopping place during an MRM.");
        oc.do_register("device.toc.mrmSafeSpotDuration", SumoOption::float(60.0));
        oc.add_description("device.toc.mrmSafeSpotDuration", "ToC Device", "Duration the vehicle stays at the safe spot after an MRM.");
        oc.do_register("device.toc.maxPreparationAccel", SumoOption::float(0.0));
        oc.add_description("device.toc.maxPreparationAccel", "ToC Device", "Maximal acceleration that may be applied during the ToC preparation phase.");
        oc.do_register("device.toc.ogNewTimeHeadway", SumoOption::float(-1.0));
        oc.add_description("device.toc.ogNewTimeHeadway", "ToC Device", "Timegap for ToC preparation phase.");
        oc.do_register("device.toc.ogNewSpaceHeadway", SumoOption::float(-1.0));
        oc.add_description("device.toc.ogNewSpaceHeadway", "ToC Device", "Additional spacing for ToC preparation phase.");
        oc.do_register("device.toc.ogMaxDecel", SumoOption::float(-1.0));
        oc.add_description("device.toc.ogMaxDecel", "ToC Device", "Maximal deceleration applied for establishing increased gap in ToC preparation phase.");
        oc.do_register("device.toc.ogChangeRate", SumoOption::float(-1.0));
        oc.add_description("device.toc.ogChangeRate", "ToC Device", "Rate of adaptation towards the increased headway during ToC preparation.");
        oc.do_register("device.toc.useColorScheme", SumoOption::bool(true));
        oc.add_description("device.toc.useColorScheme", "ToC Device", "Whether a coloring scheme shall by applied to indicate the different ToC stages.");
        oc.do_register("device.toc.file", SumoOption::string());
        oc.add_description("device.toc.file", "ToC Device", "Switches on output by specifying an output filename.");
    }

    /// C++: `void MSDevice_ToC::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        // shortcut: with_options + equipped_by_default_assignment_options are not ported, upgrade when the OptionsCont-based default-equip check lands
    if panic!("shortcut: equipped_by_default_assignment_options not ported") {
            if g_use_meso_sim() {
                utils::common::msg_handler::write_warning("ToC device is not supported by the mesoscopic simulation.");
                return;
            }
            let manualType = v.get_string_param("device.toc.manualType", true, "");
            let automatedType = v.get_string_param("device.toc.automatedType", true, "");
            let responseTime = time2steps(v.get_float_param("device.toc.responseTime", false, 0.0, false));
            let recoveryRate = v.get_float_param("device.toc.recoveryRate", false, 0.0, false);
            let lcAbstinence = v.get_float_param("device.toc.lcAbstinence", false, 0.0, false);
            let initialAwareness = v.get_float_param("device.toc.initialAwareness", false, 0.0, false);
            let mrmDecel = v.get_float_param("device.toc.mrmDecel", false, 0.0, false);
            let useColoring = v.get_bool_param("device.toc.useColorScheme", false, false);
            let file = v.get_string_param("device.toc.file", false, "");
            let ogp = Self::get_open_gap_params(v);
            let dynamicToCThreshold = v.get_float_param("device.toc.dynamicToCThreshold", false, 0.0, false);
            let dynamicMRMProbability = Self::get_dynamic_mrm_probability(v);
            let mrmKeepRight = v.get_bool_param("device.toc.mrmKeepRight", false, false);
            let mrmSafeSpot = v.get_string_param("device.toc.mrmSafeSpot", false, "");
            let mrmSafeSpotDuration = time2steps(v.get_float_param("device.toc.mrmSafeSpotDuration", false, 0.0, false));
            let maxPreparationAccel = v.get_float_param("device.toc.maxPreparationAccel", false, 0.0, false);
            // build the device
            let device: Ptr<MSDevice_ToC> = Ptr::from_mut(Box::leak(Box::new(MSDevice_ToC::new(v, &format!("toc_{}", v.get_id()), &file,
                &manualType, &automatedType, responseTime, recoveryRate,
                lcAbstinence, initialAwareness, mrmDecel, dynamicToCThreshold,
                dynamicMRMProbability, maxPreparationAccel, mrmKeepRight,
                &mrmSafeSpot, mrmSafeSpotDuration, useColoring, ogp))));
            // register at static instance container (C++ does this in the constructor; done here because
            // the Rust constructor returns by value and the address must be the final one)
            MY_INSTANCES.with(|m| m.borrow_mut().insert(device));
            into.push(device.cast::<MSVehicleDevice>());
        }
    }

    /// C++: `const int & MSDevice_ToC::getInstances()` [static]
    /// returns all currently existing ToC devices
    // static const std::set<MSDevice_ToC*, ComparatorNumericalIdLess>& getInstances() {
    //         return myInstances;
    //     }
    #[allow(unused_variables)]
    pub fn get_instances() -> &'static BTreeSet<Ptr<MSDevice_ToC>> {
        // SAFETY: thread-local storage lives for the thread; C++ returns a reference to the global set
        unsafe { &*MY_INSTANCES.with(|m| m.as_ptr()) }
    }

    /// C++: `void MSDevice_ToC::cleanup()` [static]
    /// Closes root tags of output files
    // void
    // MSDevice_ToC::cleanup() {
    //     // Close xml bodies for all existing files
    //     // TODO: Check if required
    //     for (auto& fn : createdOutputFiles) {
    //         OutputDevice* file = &OutputDevice::getDevice(fn);
    //         file->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        // Close xml bodies for all existing files
        // TODO: Check if required
        for fn_ in CREATED_OUTPUT_FILES.with(|f| f.borrow().clone()) {
            let file = Ptr::from_raw(OutputDevice::get_device(&fn_, true).unwrap_or_else(|e| panic!("{}", e)).as_ptr());
            file.m().close_tag("");
        }
    }

    /// C++: `double MSDevice_ToC::getDynamicMRMProbability(const SUMOVehicle &)` [static, private]
    /// @{
    // double
    // MSDevice_ToC::getDynamicMRMProbability(const SUMOVehicle& v) {
    //     double pMRM = v.getFloatParam("device.toc.dynamicMRMProbability");
    //     if (pMRM < 0 || pMRM > 0.5) {
    //         const double pMRMTrunc = MAX2(0.0, MIN2(0.5, pMRM));
    //         WRITE_WARNINGF(TL("Given value for ToC device parameter 'dynamicMRMProbability' (=%) is not in the admissible range [0,0.5]. Truncated to %."), toString(pMRM), toString(pMRMTrunc));
    //         return pMRMTrunc;
    //     }
    //     return pMRM;
    // }
    #[allow(unused_variables)]
    pub fn get_dynamic_mrm_probability(v: &SUMOVehicle) -> f64 {
        let pMRM = v.get_float_param("device.toc.dynamicMRMProbability", false, 0.0, false);
        if pMRM < 0.0 || pMRM > 0.5 {
            let pMRMTrunc = f64::max(0.0, f64::min(0.5, pMRM));
            write_warningf!("Given value for ToC device parameter 'dynamicMRMProbability' (=%) is not in the admissible range [0,0.5]. Truncated to %.", pMRM, pMRMTrunc);
            return pMRMTrunc;
        }
        pMRM
    }

    /// C++: `OpenGapParams MSDevice_ToC::getOpenGapParams(const SUMOVehicle &)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_open_gap_params(v: &SUMOVehicle) -> MSDevice_ToC_OpenGapParams {
        let mut timegap = v.get_float_param("device.toc.ogNewTimeHeadway", false, 0.0, false);
        let mut spacing = v.get_float_param("device.toc.ogNewSpaceHeadway", false, 0.0, false);
        let mut changeRate = v.get_float_param("device.toc.ogChangeRate", false, 0.0, false);
        let mut maxDecel = v.get_float_param("device.toc.ogMaxDecel", false, 0.0, false);
        let mut specifiedAny = false;
        if changeRate == -1.0 {
            changeRate = DEFAULT_OPENGAP_CHANGERATE;
        } else {
            specifiedAny = true;
        }
        if maxDecel == -1.0 {
            maxDecel = DEFAULT_OPENGAP_MAXDECEL;
        } else {
            specifiedAny = true;
        }
        if specifiedAny && timegap == -1.0 && spacing == -1.0 {
            utils::common::msg_handler::write_error("If any openGap parameters for the ToC model are specified, then at least one of toc.ogNewTimeHeadway and toc.ogNewSpaceHeadway must be defined.");
        }
        if timegap == -1.0 {
            timegap = DEFAULT_OPENGAP_TIMEGAP;
        } else {
            specifiedAny = true;
        }
        if spacing == -1.0 {
            spacing = DEFAULT_OPENGAP_SPACING;
        } else {
            specifiedAny = true;
        }
        MSDevice_ToC_OpenGapParams::new(timegap, spacing, changeRate, maxDecel, specifiedAny)
    }

    /// C++: `ToCState MSDevice_ToC::_2ToCState(const std::string &)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn n2_to_c_state(arg0: &str) -> MSDevice_ToC_ToCState {
        match arg0 {
            "UNDEFINED" => MSDevice_ToC_ToCState::UNDEFINED,
            "MANUAL" => MSDevice_ToC_ToCState::MANUAL,
            "AUTOMATED" => MSDevice_ToC_ToCState::AUTOMATED,
            "PREPARING_TOC" => MSDevice_ToC_ToCState::PREPARING_TOC,
            "MRM" => MSDevice_ToC_ToCState::MRM,
            "RECOVERING" => MSDevice_ToC_ToCState::RECOVERING,
            _ => {
                write_warningf!("Unknown ToCState '%'", arg0);
                MSDevice_ToC_ToCState::UNDEFINED
            }
        }
    }

    /// C++: `std::string MSDevice_ToC::_2string(ToCState)` [static, private]
    // std::string
    // MSDevice_ToC::_2string(ToCState state) {
    //     if (state == UNDEFINED) {
    //         return "UNDEFINED";
    //     } else if (state == MANUAL) {
    //         return "MANUAL";
    //     } else if (state == AUTOMATED) {
    //         return "AUTOMATED";
    //     } else if (state == PREPARING_TOC) {
    //         return "PREPARING_TOC";
    //     } else if (state == MRM) {
    //         return "MRM";
    //     } else if (state == RECOVERING) {
    //         return "RECOVERING";
    //     } else {
    //         WRITE_WARNINGF(TL("Unknown ToCState '%'"), toString(state));
    //         return toString(state);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn n2string(state: MSDevice_ToC_ToCState) -> String {
        match state {
            MSDevice_ToC_ToCState::UNDEFINED => "UNDEFINED".to_string(),
            MSDevice_ToC_ToCState::MANUAL => "MANUAL".to_string(),
            MSDevice_ToC_ToCState::AUTOMATED => "AUTOMATED".to_string(),
            MSDevice_ToC_ToCState::PREPARING_TOC => "PREPARING_TOC".to_string(),
            MSDevice_ToC_ToCState::MRM => "MRM".to_string(),
            MSDevice_ToC_ToCState::RECOVERING => "RECOVERING".to_string(),
            _ => {
                let s = (state as i64).to_string();
                write_warningf!("Unknown ToCState '%'", s);
                s
            }
        }
    }

    /// C++: `void MSDevice_ToC::~MSDevice_ToC()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // unregister from static instance container
        let me = Ptr::from_mut(self);
        MY_INSTANCES.with(|m| m.borrow_mut().remove(&me));
        // shortcut: C++ deschedules the device's WrappingCommands here; commands are not ported, so nothing is descheduled, upgrade when Command/event control is ported
        // skip resetDeliberateLCs(): vehicle is being destroyed
        self.base.drop_();
    }

    /// C++: `const std::string MSDevice_ToC::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const override {
    //         return "toc";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "toc".to_string()
    }

    /// C++: `bool MSDevice_ToC::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Return value indicates whether the device still wants to be notified about the vehicle movement
    // bool
    // MSDevice_ToC::notifyMove(SUMOTrafficObject& /*veh*/,
    //                          double /*oldPos*/,
    //                          double /*newPos*/,
    //                          double /*newSpeed*/) {
    //     if (myState == AUTOMATED && checkDynamicToC()) {
    //         // Initiate a ToC
    //         // Record event
    //         if (generatesOutput()) {
    //             myEvents.push(std::make_pair(SIMSTEP, "DYNTOR"));
    //             myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //             myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //         }
    //         // Leadtime for dynamic ToC is proportional to the time assumed for the dynamic ToC threshold
    //         const double leadTime = myDynamicToCThreshold * 1000 * DYNAMIC_TOC_LEADTIME_FACTOR;
    //         requestToC((SUMOTime) leadTime);
    //         // TODO: Alter the response time according to the given lead time. Consider re-sampling it at each call of requestToC(). (Conditional to whether a non-negative response time was given in the configuration)
    //         myIssuedDynamicToC = true;
    //         myDynamicToCLane = myHolderMS->getLane()->getNumericalID();
    //     } else if (myIssuedDynamicToC && myState == PREPARING_TOC && !checkDynamicToC()) {
    //         // Abort dynamic ToC, FIXME: This could abort an externally requested ToC in rare occasions... (needs test)
    //         // Record event
    //         if (generatesOutput()) {
    //             myEvents.push(std::make_pair(SIMSTEP, "DYNTOR"));
    //             myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //             myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //         }
    //         // NOTE: This should not occur if lane changing is prevented during ToC preparation...
    //         // TODO: Reset response time to the original value (unnecessary if re-sampling for each call to requestToC)
    //         triggerUpwardToC(0);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        if self.myState == MSDevice_ToC_ToCState::AUTOMATED && self.check_dynamic_to_c() {
            // Initiate a ToC
            // Record event
            if self.generates_output() {
                self.record_event(MSNet::get_instance().get_current_time_step(), "DYNTOR");
            }
            // Leadtime for dynamic ToC is proportional to the time assumed for the dynamic ToC threshold
            let leadTime = self.myDynamicToCThreshold * 1000.0 * DYNAMIC_TOC_LEADTIME_FACTOR;
            self.request_to_c(leadTime as SUMOTime, -1000);
            // TODO: Alter the response time according to the given lead time. Consider re-sampling it at each call of requestToC(). (Conditional to whether a non-negative response time was given in the configuration)
            self.myIssuedDynamicToC = true;
            self.myDynamicToCLane = self.myHolderMS.get_lane().get_numerical_id();
        } else if self.myIssuedDynamicToC && self.myState == MSDevice_ToC_ToCState::PREPARING_TOC && !self.check_dynamic_to_c() {
            // Abort dynamic ToC, FIXME: This could abort an externally requested ToC in rare occasions... (needs test)
            // Record event
            if self.generates_output() {
                self.record_event(MSNet::get_instance().get_current_time_step(), "DYNTOR");
            }
            // NOTE: This should not occur if lane changing is prevented during ToC preparation...
            // TODO: Reset response time to the original value (unnecessary if re-sampling for each call to requestToC)
            self.trigger_upward_to_c(0);
        }
        true
    }

    /// C++: `std::string MSDevice_ToC::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_ToC::getParameter(const std::string& key) const {
    //     if (key == "manualType") {
    //         return myManualTypeID;
    //     } else if (key == "automatedType") {
    //         return myAutomatedTypeID;
    //     } else if (key == "responseTime") {
    //         return toString(STEPS2TIME(myResponseTime));
    //     } else if (key == "recoveryRate") {
    //         return toString(myRecoveryRate);
    //     } else if (key == "initialAwareness") {
    //         return toString(myInitialAwareness);
    //     } else if (key == "mrmDecel") {
    //         return toString(myMRMDecel);
    //     } else if (key == "currentAwareness") {
    //         return toString(myCurrentAwareness);
    //     } else if (key == "lcAbstinence") {
    //         return toString(myLCAbstinence);
    //     } else if (key == "state") {
    //         return _2string(myState);
    //     } else if (key == "holder") {
    //         return myHolder.getID();
    //     } else if (key == "hasDynamicToC") {
    //         return toString(myDynamicToCActive);
    //     } else if (key == "dynamicToCThreshold") {
    //         return toString(myDynamicToCThreshold);
    //     } else if (key == "dynamicMRMProbability") {
    //         return toString(myMRMProbability);
    //     } else if (key == "mrmKeepRight") {
    //         return toString(myMRMKeepRight);
    //     } else if (key == "mrmSafeSpot") {
    //         return myMRMSafeSpot;
    //     } else if (key == "mrmSafeSpotDuration") {
    //         return toString(STEPS2TIME(myMRMSafeSpotDuration));
    //     } else if (key == "maxPreparationAccel") {
    //         return toString(myMaxPreparationAccel);
    //     } else if (key == "ogNewTimeHeadway") {
    //         return toString(myOpenGapParams.newTimeHeadway);
    //     } else if (key == "ogNewSpaceHeadway") {
    //         return toString(myOpenGapParams.newSpaceHeadway);
    //     } else if (key == "ogChangeRate") {
    //         return toString(myOpenGapParams.changeRate);
    //     } else if (key == "ogMaxDecel") {
    //         return toString(myOpenGapParams.maxDecel);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        if key == "manualType" {
            self.myManualTypeID.clone()
        } else if key == "automatedType" {
            self.myAutomatedTypeID.clone()
        } else if key == "responseTime" {
            utils::common::to_string::to_string(&steps2time(self.myResponseTime))
        } else if key == "recoveryRate" {
            utils::common::to_string::to_string(&self.myRecoveryRate)
        } else if key == "initialAwareness" {
            utils::common::to_string::to_string(&self.myInitialAwareness)
        } else if key == "mrmDecel" {
            utils::common::to_string::to_string(&self.myMRMDecel)
        } else if key == "currentAwareness" {
            utils::common::to_string::to_string(&self.myCurrentAwareness)
        } else if key == "lcAbstinence" {
            utils::common::to_string::to_string(&self.myLCAbstinence)
        } else if key == "state" {
            Self::n2string(self.myState)
        } else if key == "holder" {
            self.myHolder.get_id().to_string()
        } else if key == "hasDynamicToC" {
            self.myDynamicToCActive.to_string()
        } else if key == "dynamicToCThreshold" {
            utils::common::to_string::to_string(&self.myDynamicToCThreshold)
        } else if key == "dynamicMRMProbability" {
            utils::common::to_string::to_string(&self.myMRMProbability)
        } else if key == "mrmKeepRight" {
            self.myMRMKeepRight.to_string()
        } else if key == "mrmSafeSpot" {
            self.myMRMSafeSpot.clone()
        } else if key == "mrmSafeSpotDuration" {
            utils::common::to_string::to_string(&steps2time(self.myMRMSafeSpotDuration))
        } else if key == "maxPreparationAccel" {
            utils::common::to_string::to_string(&self.myMaxPreparationAccel)
        } else if key == "ogNewTimeHeadway" {
            utils::common::to_string::to_string(&self.myOpenGapParams.newTimeHeadway)
        } else if key == "ogNewSpaceHeadway" {
            utils::common::to_string::to_string(&self.myOpenGapParams.newSpaceHeadway)
        } else if key == "ogChangeRate" {
            utils::common::to_string::to_string(&self.myOpenGapParams.changeRate)
        } else if key == "ogMaxDecel" {
            utils::common::to_string::to_string(&self.myOpenGapParams.maxDecel)
        } else {
            panic!("{}", ProcessError::InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())));
        }
    }

    /// C++: `void MSDevice_ToC::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_ToC::setParameter(const std::string& key, const std::string& value) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " MSDevice_ToC::setParameter(key=" << key << ", value=" << value << ")" << std::endl;
    // #endif
    //     if (key == "manualType") {
    //         myManualTypeID = value;
    //         myColorScheme[MANUAL] = MSNet::getInstance()->getVehicleControl().getVType(myManualTypeID)->getColor();
    //         if (myState == MANUAL) {
    //             switchHolderType(value);
    //         }
    //     } else if (key == "automatedType") {
    //         myAutomatedTypeID = value;
    //         myColorScheme[AUTOMATED] = MSNet::getInstance()->getVehicleControl().getVType(myAutomatedTypeID)->getColor();
    //         if (myState == AUTOMATED || myState == PREPARING_TOC || myState == MRM) {
    //             switchHolderType(value);
    //         }
    //     } else if (key == "responseTime") {
    //         myResponseTime = TIME2STEPS(StringUtils::toDouble(value));
    //     } else if (key == "recoveryRate") {
    //         myRecoveryRate = StringUtils::toDouble(value);
    //     } else if (key == "initialAwareness") {
    //         myInitialAwareness = StringUtils::toDouble(value);
    //     } else if (key == "lcAbstinence") {
    //         myLCAbstinence = StringUtils::toDouble(value);
    //         if (isManuallyDriven()) {
    //             setAwareness(myCurrentAwareness); // to eventually trigger LC-prevention
    //         }
    //     } else if (key == "currentAwareness") {
    //         if (isManuallyDriven()) {
    //             setAwareness(StringUtils::toDouble(value));
    //         } else {
    //             WRITE_WARNING(TL("Setting device.toc.currentAwareness during automated mode has no effect."))
    //         }
    //     } else if (key == "mrmDecel") {
    //         myMRMDecel = StringUtils::toDouble(value);
    //     } else if (key == "requestToC") {
    //         // setting this magic parameter gives the interface for inducing a ToC
    //         const SUMOTime timeTillMRM = TIME2STEPS(StringUtils::toDouble(value));
    //         requestToC(timeTillMRM, myResponseTime);
    //     } else if (key == "requestMRM") {
    //         // setting this magic parameter gives the interface for inducing an MRM
    //         requestMRM();
    //     } else if (key == "awareness") {
    //         // setting this magic parameter gives the interface for setting the driverstate's awareness
    //         setAwareness(StringUtils::toDouble(value));
    //     } else if (key == "dynamicToCThreshold") {
    //         const double newValue = StringUtils::toDouble(value);
    //         if (newValue < 0) {
    //             WRITE_WARNINGF(TL("Value of dynamicToCThreshold must be non-negative. (Given value % for vehicle % is ignored)"), value, myHolderMS->getID());
    //         } else if (newValue == 0) {
    //             myDynamicToCThreshold = newValue;
    //             myDynamicToCActive = false;
    //         } else {
    //             myDynamicToCThreshold = newValue;
    //             myDynamicToCActive = true;
    //         }
    //     } else if (key == "dynamicMRMProbability") {
    //         const double newValue = StringUtils::toDouble(value);
    //         if (newValue < 0) {
    //             WRITE_WARNINGF(TL("Value of dynamicMRMProbability must be non-negative. (Given value % for vehicle % is ignored)"), value, myHolderMS->getID());
    //         } else {
    //             myMRMProbability = newValue;
    //         }
    //     } else if (key == "mrmKeepRight")  {
    //         const bool newValue = StringUtils::toBool(value);
    //         myMRMKeepRight = newValue;
    //     } else if (key == "mrmSafeSpot") {
    //         myMRMSafeSpot = value;
    //     } else if (key == "mrmSafeSpotDuration") {
    //         myMRMSafeSpotDuration = TIME2STEPS(StringUtils::toDouble(value));
    //     } else if (key == "maxPreparationAccel") {
    //         const double newValue = StringUtils::toDouble(value);
    //         if (newValue < 0) {
    //             WRITE_WARNINGF(TL("Value of maxPreparationAccel must be non-negative. (Given value % for vehicle % is ignored)"), value, myHolderMS->getID());
    //         } else {
    //             myMaxPreparationAccel = newValue;
    //         }
    //     } else if (key == "ogNewTimeHeadway") {
    //         const double newValue = StringUtils::toDouble(value);
    //         myOpenGapParams.newTimeHeadway = newValue;
    //         myOpenGapParams.active = true;
    //     } else if (key == "ogNewSpaceHeadway") {
    //         const double newValue = StringUtils::toDouble(value);
    //         myOpenGapParams.newSpaceHeadway = newValue;
    //         myOpenGapParams.active = true;
    //     } else if (key == "ogChangeRate") {
    //         const double newValue = StringUtils::toDouble(value);
    //         myOpenGapParams.changeRate = newValue;
    //         myOpenGapParams.active = true;
    //     } else if (key == "ogMaxDecel") {
    //         const double newValue = StringUtils::toDouble(value);
    //         myOpenGapParams.maxDecel = newValue;
    //         myOpenGapParams.active = true;
    //     } else {
    //         throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        let parse = |v: &str| -> f64 {
            v.parse::<f64>().unwrap_or_else(|_| panic!("{}", ProcessError::new(format!("Invalid number '{}'", v))))
        };
        if key == "manualType" {
            self.myManualTypeID = value.to_string();
            let c = MSNet::get_instance().m().get_vehicle_control().get_v_type(&self.myManualTypeID, SUMORouteHandler::get_parsing_rng(), false).get_color().clone();
            self.myColorScheme.insert(MSDevice_ToC_ToCState::MANUAL, c);
            if self.myState == MSDevice_ToC_ToCState::MANUAL {
                self.switch_holder_type(value);
            }
        } else if key == "automatedType" {
            self.myAutomatedTypeID = value.to_string();
            let c = MSNet::get_instance().m().get_vehicle_control().get_v_type(&self.myAutomatedTypeID, SUMORouteHandler::get_parsing_rng(), false).get_color().clone();
            self.myColorScheme.insert(MSDevice_ToC_ToCState::AUTOMATED, c);
            if self.myState == MSDevice_ToC_ToCState::AUTOMATED || self.myState == MSDevice_ToC_ToCState::PREPARING_TOC || self.myState == MSDevice_ToC_ToCState::MRM {
                self.switch_holder_type(value);
            }
        } else if key == "responseTime" {
            self.myResponseTime = time2steps(parse(value));
        } else if key == "recoveryRate" {
            self.myRecoveryRate = parse(value);
        } else if key == "initialAwareness" {
            self.myInitialAwareness = parse(value);
        } else if key == "lcAbstinence" {
            self.myLCAbstinence = parse(value);
            if self.is_manually_driven() {
                self.set_awareness(self.myCurrentAwareness); // to eventually trigger LC-prevention
            }
        } else if key == "currentAwareness" {
            if self.is_manually_driven() {
                self.set_awareness(parse(value));
            } else {
                utils::common::msg_handler::write_warning("Setting device.toc.currentAwareness during automated mode has no effect.");
            }
        } else if key == "mrmDecel" {
            self.myMRMDecel = parse(value);
        } else if key == "requestToC" {
            // setting this magic parameter gives the interface for inducing a ToC
            let timeTillMRM = time2steps(parse(value));
            self.request_to_c(timeTillMRM, self.myResponseTime);
        } else if key == "requestMRM" {
            // setting this magic parameter gives the interface for inducing an MRM
            self.request_mrm();
        } else if key == "awareness" {
            // setting this magic parameter gives the interface for setting the driverstate's awareness
            self.set_awareness(parse(value));
        } else if key == "dynamicToCThreshold" {
            let newValue = parse(value);
            if newValue < 0.0 {
                write_warningf!("Value of dynamicToCThreshold must be non-negative. (Given value % for vehicle % is ignored)", value, self.myHolder.get_id());
            } else if newValue == 0.0 {
                self.myDynamicToCThreshold = newValue;
                self.myDynamicToCActive = false;
            } else {
                self.myDynamicToCThreshold = newValue;
                self.myDynamicToCActive = true;
            }
        } else if key == "dynamicMRMProbability" {
            let newValue = parse(value);
            if newValue < 0.0 {
                write_warningf!("Value of dynamicMRMProbability must be non-negative. (Given value % for vehicle % is ignored)", value, self.myHolder.get_id());
            } else {
                self.myMRMProbability = newValue;
            }
        } else if key == "mrmKeepRight" {
            let newValue = utils::common::string_utils::to_bool(value).unwrap_or_else(|e| panic!("{}", e));
            self.myMRMKeepRight = newValue;
        } else if key == "mrmSafeSpot" {
            self.myMRMSafeSpot = value.to_string();
        } else if key == "mrmSafeSpotDuration" {
            self.myMRMSafeSpotDuration = time2steps(parse(value));
        } else if key == "maxPreparationAccel" {
            let newValue = parse(value);
            if newValue < 0.0 {
                write_warningf!("Value of maxPreparationAccel must be non-negative. (Given value % for vehicle % is ignored)", value, self.myHolder.get_id());
            } else {
                self.myMaxPreparationAccel = newValue;
            }
        } else if key == "ogNewTimeHeadway" {
            self.myOpenGapParams.newTimeHeadway = parse(value);
            self.myOpenGapParams.active = true;
        } else if key == "ogNewSpaceHeadway" {
            self.myOpenGapParams.newSpaceHeadway = parse(value);
            self.myOpenGapParams.active = true;
        } else if key == "ogChangeRate" {
            self.myOpenGapParams.changeRate = parse(value);
            self.myOpenGapParams.active = true;
        } else if key == "ogMaxDecel" {
            self.myOpenGapParams.maxDecel = parse(value);
            self.myOpenGapParams.active = true;
        } else {
            panic!("{}", ProcessError::InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())));
        }
    }

    /// C++: `SUMOTime MSDevice_ToC::triggerMRM(SUMOTime)`
    /// Trigger execution of an MRM
    // SUMOTime
    // MSDevice_ToC::triggerMRM(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " triggerMRM() for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     // Clear ongoing MRM
    //     descheduleMRM();
    //
    //     // Start MRM process
    //     if (myMRMSafeSpot != "") {
    //         SUMOVehicleParameter::Stop stop;
    //         MSStoppingPlace* s = MSNet::getInstance()->getStoppingPlace(myMRMSafeSpot, SUMO_TAG_PARKING_AREA);
    //         if (s == nullptr) {
    //             WRITE_WARNINGF(TL("Ignoring unknown safe spot '%' for vehicle '%'."), myMRMSafeSpot, myHolder.getID());
    //         } else {
    //             stop.parkingarea = myMRMSafeSpot;
    //             stop.parking = ParkingType::OFFROAD;
    //             stop.lane = s->getLane().getID();
    //             stop.endPos = s->getEndLanePosition();
    //             stop.startPos = s->getBeginLanePosition();
    //             stop.duration = myMRMSafeSpotDuration;
    //             myHolderMS->getSingularType().setDecel(myMRMDecel);
    //             std::string error;
    //             if (!myHolder.addStop(stop, error)) {
    //                 WRITE_WARNING("Could not set safe spot '" + myMRMSafeSpot + "' for vehicle '" + myHolder.getID() + "'. " + error);
    //             }
    //         }
    //     } else {
    //         myExecuteMRMCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::MRMExecutionStep);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myExecuteMRMCommand, SIMSTEP + DELTA_T);
    //     }
    //     if (myState == MANUAL || myState == RECOVERING) {
    //         switchHolderType(myAutomatedTypeID);
    //     }
    //     setState(MRM);
    //     setAwareness(1.);
    //
    //     // Record event
    //     if (generatesOutput()) {
    //         myEvents.push(std::make_pair(SIMSTEP, "MRM"));
    //         myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //         myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //     }
    //
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn trigger_mrm(&mut self, t: SUMOTime) -> SUMOTime {
        // Clear ongoing MRM
        self.deschedule_mrm();
        // Start MRM process
        if self.myMRMSafeSpot != "" {
            let mut stop: SUMOVehicleParameter_Stop = SUMOVehicleParameter_Stop::default();
            let s = MSNet::get_instance().m().get_stopping_place(&self.myMRMSafeSpot, SUMO_TAG_PARKING_AREA);
            if s.is_null() {
                write_warningf!("Ignoring unknown safe spot '%' for vehicle '%'.", self.myMRMSafeSpot, self.myHolder.get_id());
            } else {
                stop.parkingarea = self.myMRMSafeSpot.clone();
                stop.parking = ParkingType::OFFROAD;
                stop.lane = s.get_lane().get_id().to_string();
                stop.endPos = s.get_end_lane_position();
                stop.startPos = s.get_begin_lane_position();
                stop.duration = self.myMRMSafeSpotDuration;
                self.myHolderMS.m().get_singular_type().set_decel(self.myMRMDecel);
                let mut error = String::new();
                let holderPtr = self.myHolder;
                if !holderPtr.m().add_stop(&stop, &mut error, 0, Ptr::null()) {
                    utils::common::msg_handler::write_warning(&format!("Could not set safe spot '{}' for vehicle '{}'. {}", self.myMRMSafeSpot, self.myHolder.get_id(), error));
                }
            }
        } else {
            // shortcut: C++ schedules MRMExecutionStep as a WrappingCommand at begin-of-timestep events; commands are not ported, so the MRM execution step is not scheduled here, upgrade when Command/event control is ported
        }
        if self.myState == MSDevice_ToC_ToCState::MANUAL || self.myState == MSDevice_ToC_ToCState::RECOVERING {
            let target = self.myAutomatedTypeID.clone();
            self.switch_holder_type(&target);
        }
        self.set_state(MSDevice_ToC_ToCState::MRM);
        self.set_awareness(1.0);
        // Record event
        if self.generates_output() {
            self.record_event(MSNet::get_instance().get_current_time_step(), "MRM");
        }
        0
    }

    /// C++: `SUMOTime MSDevice_ToC::triggerUpwardToC(SUMOTime)`
    /// Trigger execution of a ToC X-->AUTOMATED ("upwards")
    // SUMOTime
    // MSDevice_ToC::triggerUpwardToC(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " triggerUpwardToC() for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     descheduleToC();
    //     // Eventually stop ToC preparation process
    //     descheduleToCPreparation();
    //     // Eventually abort MRM
    //     descheduleMRM();
    //     // Eventually abort awareness recovery process
    //     descheduleRecovery();
    //
    //     if (myState == MANUAL || myState == RECOVERING) {
    //         switchHolderType(myAutomatedTypeID);
    //     }
    //     setAwareness(1.);
    //     setState(AUTOMATED);
    //
    //     // Record event
    //     if (generatesOutput()) {
    //         myEvents.push(std::make_pair(SIMSTEP, "ToCup"));
    //         myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //         myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //     }
    //
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn trigger_upward_to_c(&mut self, t: SUMOTime) -> SUMOTime {
        self.deschedule_to_c();
        // Eventually stop ToC preparation process
        self.deschedule_to_c_preparation();
        // Eventually abort MRM
        self.deschedule_mrm();
        // Eventually abort awareness recovery process
        self.deschedule_recovery();
        if self.myState == MSDevice_ToC_ToCState::MANUAL || self.myState == MSDevice_ToC_ToCState::RECOVERING {
            let target = self.myAutomatedTypeID.clone();
            self.switch_holder_type(&target);
        }
        self.set_awareness(1.0);
        self.set_state(MSDevice_ToC_ToCState::AUTOMATED);
        // Record event
        if self.generates_output() {
            self.record_event(MSNet::get_instance().get_current_time_step(), "ToCup");
        }
        0
    }

    /// C++: `SUMOTime MSDevice_ToC::triggerDownwardToC(SUMOTime)`
    /// Trigger execution of a ToC X-->MANUAL ("downwards")
    // SUMOTime
    // MSDevice_ToC::triggerDownwardToC(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " triggerDownwardToC() for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     descheduleToC();
    //     // Eventually stop ToC preparation process
    //     descheduleToCPreparation();
    //     // Eventually abort MRM
    //     descheduleMRM();
    //
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " Initial awareness after ToC: " << myCurrentAwareness << std::endl;
    // #endif
    //
    //     // Start awareness recovery process
    //     myRecoverAwarenessCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::awarenessRecoveryStep);
    //     MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myRecoverAwarenessCommand, SIMSTEP + DELTA_T);
    //     setState(RECOVERING);
    //
    //     // @todo: Sample initial awareness
    //     double initialAwareness = myInitialAwareness;
    //     setAwareness(initialAwareness);
    //
    //     switchHolderType(myManualTypeID);
    //
    //     // Record event
    //     if (generatesOutput()) {
    //         myEvents.push(std::make_pair(SIMSTEP, "ToCdown"));
    //         myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //         myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn trigger_downward_to_c(&mut self, t: SUMOTime) -> SUMOTime {
        self.deschedule_to_c();
        // Eventually stop ToC preparation process
        self.deschedule_to_c_preparation();
        // Eventually abort MRM
        self.deschedule_mrm();
        // Start awareness recovery process
        // shortcut: C++ schedules awarenessRecoveryStep as a WrappingCommand at begin-of-timestep events; commands are not ported, upgrade when Command/event control is ported
        self.set_state(MSDevice_ToC_ToCState::RECOVERING);
        // @todo: Sample initial awareness
        let initialAwareness = self.myInitialAwareness;
        self.set_awareness(initialAwareness);
        let target = self.myManualTypeID.clone();
        self.switch_holder_type(&target);
        // Record event
        if self.generates_output() {
            self.record_event(MSNet::get_instance().get_current_time_step(), "ToCdown");
        }
        0
    }

    /// C++: `SUMOTime MSDevice_ToC::ToCPreparationStep(SUMOTime)`
    /// Continue the ToC preparation for one time step
    // SUMOTime
    // MSDevice_ToC::ToCPreparationStep(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " ToC preparation step for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     if (myState == PREPARING_TOC) {
    //         return DELTA_T;
    //     } else {
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " Aborting ToC preparation for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //         descheduleToCPreparation();
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn to_c_preparation_step(&mut self, t: SUMOTime) -> SUMOTime {
        if self.myState == MSDevice_ToC_ToCState::PREPARING_TOC {
            delta_t()
        } else {
            self.deschedule_to_c_preparation();
            0
        }
    }

    /// C++: `SUMOTime MSDevice_ToC::MRMExecutionStep(SUMOTime)`
    /// Continue the MRM for one time step
    // SUMOTime
    // MSDevice_ToC::MRMExecutionStep(SUMOTime t) {
    //     deactivateDeliberateLCs();
    //     const double currentSpeed = myHolderMS->getSpeed();
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " MRM step for vehicle '" << myHolder.getID() << "', currentSpeed=" << currentSpeed << std::endl;
    // #endif
    //
    //     // Induce slowdown with MRMDecel
    //     if (currentSpeed > SPEED_EPS) {
    //         std::vector<std::pair<SUMOTime, double> > speedTimeLine;
    //         const double nextSpeed = MAX2(0., currentSpeed - ACCEL2SPEED(myMRMDecel));
    //         speedTimeLine.push_back(std::make_pair(t, currentSpeed));
    //         speedTimeLine.push_back(std::make_pair(t + DELTA_T, nextSpeed));
    //         myHolderMS->getInfluencer().setSpeedTimeLine(speedTimeLine);
    //     }
    //
    //     if (myMRMKeepRight) {
    //         // Try to change to the right
    //         std::vector<std::pair<SUMOTime, int> > laneTimeLine;
    //         laneTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), 0));
    //         laneTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + 1, 0));
    //         myHolderMS->getInfluencer().setLaneTimeLine(laneTimeLine);
    // //    	std::cout << "Keeping right..." << std::endl;
    //     }
    //
    //     if (myState == MRM) {
    //         return DELTA_T;
    //     } else {
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " Aborting MRM for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //         resetDeliberateLCs();
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn mrm_execution_step(&mut self, t: SUMOTime) -> SUMOTime {
        self.deactivate_deliberate_l_cs();
        let holderPtr = self.myHolderMS;
        let currentSpeed = holderPtr.m().get_speed();
        // Induce slowdown with MRMDecel
        if currentSpeed > SPEED_EPS {
            let mut speedTimeLine: Vec<(SUMOTime, f64)> = Vec::new();
            let nextSpeed = f64::max(0.0, currentSpeed - accel2speed(self.myMRMDecel));
            speedTimeLine.push((t, currentSpeed));
            speedTimeLine.push((t + delta_t(), nextSpeed));
            holderPtr.m().get_influencer().set_speed_time_line(&speedTimeLine);
        }
        if self.myMRMKeepRight {
            // Try to change to the right
            let mut laneTimeLine: Vec<(SUMOTime, i32)> = Vec::new();
            laneTimeLine.push((MSNet::get_instance().get_current_time_step(), 0));
            laneTimeLine.push((MSNet::get_instance().get_current_time_step() + 1, 0));
            holderPtr.m().get_influencer().set_lane_time_line(&laneTimeLine);
        }
        if self.myState == MSDevice_ToC_ToCState::MRM {
            delta_t()
        } else {
            self.reset_deliberate_l_cs();
            0
        }
    }

    /// C++: `SUMOTime MSDevice_ToC::awarenessRecoveryStep(SUMOTime)`
    /// Continue the awareness recovery for one time step
    // SUMOTime
    // MSDevice_ToC::awarenessRecoveryStep(SUMOTime /* t */) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " Awareness recovery step for vehicle '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     // Proceed with awareness recovery
    //     if (myCurrentAwareness < 1.0) {
    //         setAwareness(MIN2(1.0, myCurrentAwareness + TS * myRecoveryRate));
    //     }
    //
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " currentAwareness = " << myCurrentAwareness << std::endl;
    // #endif
    //
    //     const bool awarenessRecoveryCompleted = myCurrentAwareness == 1.0;
    //     if (awarenessRecoveryCompleted) {
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " Awareness recovery completed for veh '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //         myRecoverAwarenessCommand->deschedule();
    //         myRecoverAwarenessCommand = nullptr;
    //         setState(MANUAL);
    //         return 0;
    //     }
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn awareness_recovery_step(&mut self, t: SUMOTime) -> SUMOTime {
        // Proceed with awareness recovery
        if self.myCurrentAwareness < 1.0 {
            let v = f64::min(1.0, self.myCurrentAwareness + ts() * self.myRecoveryRate);
            self.set_awareness(v);
        }
        let awarenessRecoveryCompleted = self.myCurrentAwareness == 1.0;
        if awarenessRecoveryCompleted {
            // shortcut: C++ deschedules the recovery WrappingCommand here; commands are not ported, upgrade when Command/event control is ported
            self.set_state(MSDevice_ToC_ToCState::MANUAL);
            return 0;
        }
        delta_t()
    }

    /// C++: `void MSDevice_ToC::writeOutput()`
    /// Write output to file given by option device.toc.file
    // void
    // MSDevice_ToC::writeOutput() {
    //     if (!generatesOutput()) {
    //         assert(myEvents.empty());
    //         return;
    //     }
    //     while (!myEvents.empty()) {
    //         const std::pair<SUMOTime, std::string> e = myEvents.front(); // make a copy, it is used after pop
    //         const std::pair<std::string, double>& l = myEventLanes.front();
    //         const std::pair<double, double>& p = myEventXY.front();
    //         myOutputFile->openTag(e.second);
    //         myOutputFile->writeAttr("id", myHolder.getID()).writeAttr("t", STEPS2TIME(e.first));
    //         myOutputFile->writeAttr("lane", l.first).writeAttr("lanePos", l.second);
    //         myOutputFile->writeAttr("x", p.first).writeAttr("y", p.second);
    //         myOutputFile->closeTag();
    //         myEvents.pop();
    //         myEventLanes.pop();
    //         myEventXY.pop();
    //
    //         if (e.second.compare("DYNTOR") == 0 && !myEvents.empty()) { // skip "TOR" events if duplicate of "DYNTOR"
    //             std::pair<SUMOTime, std::string>& eNext = myEvents.front();
    //             if (eNext.second.compare("TOR") == 0 && eNext.first == e.first) {
    //                 myEvents.pop();
    //                 myEventLanes.pop();
    //                 myEventXY.pop();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_output(&mut self) {
        if !self.generates_output() {
            assert!(self.myEvents.is_empty());
            return;
        }
        while let Some((eTime, eName)) = self.myEvents.front().cloned() {
            let (lane, lanePos) = self.myEventLanes.front().cloned().unwrap();
            let (x, y) = self.myEventXY.front().cloned().unwrap();
            let out = self.myOutputFile.m();
            out.open_tag(&eName);
            let id = self.myHolder.get_id().to_string();
            out.write_attr("id", id.as_str()).write_attr("t", &steps2time(eTime));
            out.write_attr("lane", lane.as_str()).write_attr("lanePos", &lanePos);
            out.write_attr("x", &x).write_attr("y", &y);
            out.close_tag("");
            self.myEvents.pop_front();
            self.myEventLanes.pop_front();
            self.myEventXY.pop_front();
            if eName == "DYNTOR" && !self.myEvents.is_empty() {
                // skip "TOR" events if duplicate of "DYNTOR"
                let (nextTime, nextName) = self.myEvents.front().cloned().unwrap();
                if nextName == "TOR" && nextTime == eTime {
                    self.myEvents.pop_front();
                    self.myEventLanes.pop_front();
                    self.myEventXY.pop_front();
                }
            }
        }
    }

    /// C++: `bool MSDevice_ToC::generatesOutput()`
    /// Whether this device requested to write output
    // bool generatesOutput() {
    //         return myOutputFile != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn generates_output(&mut self) -> bool {
        !self.myOutputFile.is_null()
    }

    /// C++: `SumoRNG * MSDevice_ToC::getResponseTimeRNG()` [static]
    // static SumoRNG* getResponseTimeRNG() {
    //         return &myResponseTimeRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_response_time_rng() -> Ptr<SumoRNG> {
        MY_RESPONSE_TIME_RNG.with(|r| Ptr::from_mut(unsafe { &mut *r.as_ptr() }))
    }

    /// C++: `void MSDevice_ToC::MSDevice_ToC(SUMOVehicle &, const std::string &, const std::string &, const std::string &, const std::string &, SUMOTime, double, double, double, double, double, double, double, bool, const std::string &, SUMOTime, bool, OpenGapParams)` [private]
    /// Constructor
    // MSDevice_ToC::MSDevice_ToC(SUMOVehicle& holder, const std::string& id, const std::string& outputFilename,
    //                            const std::string& manualType, const std::string& automatedType, SUMOTime responseTime, double recoveryRate,
    //                            double lcAbstinence, double initialAwareness, double mrmDecel,
    //                            double dynamicToCThreshold, double dynamicMRMProbability, double maxPreparationAccel,
    //                            bool mrmKeepRight, const std::string& mrmSafeSpot, SUMOTime mrmSafeSpotDuration, bool useColorScheme, OpenGapParams ogp) :
    //     MSVehicleDevice(holder, id),
    //     myManualTypeID(manualType),
    //     myAutomatedTypeID(automatedType),
    //     myResponseTime(responseTime),
    //     myRecoveryRate(recoveryRate),
    //     myLCAbstinence(lcAbstinence),
    //     myInitialAwareness(initialAwareness),
    //     myMRMDecel(mrmDecel),
    //     myCurrentAwareness(1.),
    //     myUseColorScheme(useColorScheme),
    //     myTriggerMRMCommand(nullptr),
    //     myTriggerToCCommand(nullptr),
    //     myRecoverAwarenessCommand(nullptr),
    //     myExecuteMRMCommand(nullptr),
    //     myPrepareToCCommand(nullptr),
    //     myOutputFile(nullptr),
    //     myEvents(),
    //     myEventLanes(),
    //     myEventXY(),
    //     myPreviousLCMode(-1),
    //     myOpenGapParams(ogp),
    //     myDynamicToCThreshold(dynamicToCThreshold),
    //     myMRMProbability(dynamicMRMProbability),
    //     myDynamicToCActive(dynamicToCThreshold > 0),
    //     myIssuedDynamicToC(false),
    //     myDynamicToCLane(-1),
    //     myMRMKeepRight(mrmKeepRight),
    //     myMRMSafeSpot(mrmSafeSpot),
    //     myMRMSafeSpotDuration(mrmSafeSpotDuration),
    //     myMaxPreparationAccel(maxPreparationAccel),
    //     myOriginalMaxAccel(-1) {
    //     // Take care! Holder is currently being constructed. Cast occurs before completion.
    //     myHolderMS = static_cast<MSVehicle*>(&holder);
    //
    //     if (outputFilename != "") {
    //         myOutputFile = &OutputDevice::getDevice(outputFilename);
    //         // TODO: make xsd, include header
    //         // myOutputFile.writeXMLHeader("ToCDeviceLog", "ToCDeviceLog.xsd");
    //         if (createdOutputFiles.count(outputFilename) == 0) {
    //             myOutputFile->writeXMLHeader("ToCDeviceLog", "");
    //             createdOutputFiles.insert(outputFilename);
    //         }
    //     }
    //
    //     // Check if the given vTypes for the ToC Device are vTypeDistributions
    //     MSVehicleControl& vehCtrl = MSNet::getInstance()->getVehicleControl();
    //     const bool automatedVTypeIsDist = vehCtrl.hasVTypeDistribution(myAutomatedTypeID);
    //     const bool manualVTypeIsDist = vehCtrl.hasVTypeDistribution(myManualTypeID);
    //
    //     // Check if the vType of the holder matches one of the given vTypes
    //     std::string holderVTypeID = holder.getVehicleType().getID();
    //     if (holderVTypeID == myManualTypeID) {
    //         myState = ToCState::MANUAL;
    //     } else if (holderVTypeID == myAutomatedTypeID) {
    //         myState = ToCState::AUTOMATED;
    //     } else if (manualVTypeIsDist && vehCtrl.getVTypeDistributionMembership(holderVTypeID).count(myManualTypeID) > 0) {
    //         // Holder type id is from the given manual type distribution.
    //         myState = ToCState::MANUAL;
    //         myManualTypeID = holderVTypeID;
    //     } else if (automatedVTypeIsDist && vehCtrl.getVTypeDistributionMembership(holderVTypeID).count(myAutomatedTypeID) > 0) {
    //         // Holder type id is from the given automated type distribution.
    //         myState = ToCState::AUTOMATED;
    //         myAutomatedTypeID = holderVTypeID;
    //     } else {
    //         throw ProcessError("Vehicle type of vehicle '" + holder.getID() + "' ('" +
    //                            holder.getVehicleType().getID() + "') must coincide with manualType ('" +
    //                            manualType + "') or automatedType ('" + automatedType +
    //                            "') specified for its ToC-device (or drawn from the specified vTypeDistributions).");
    //     }
    //     if (!vehCtrl.hasVType(myAutomatedTypeID)) {
    //         throw ProcessError("The automated vehicle type '" + myAutomatedTypeID +
    //                            "' of vehicle '" + holder.getID() + "' is not known.");
    //     }
    //     if (!vehCtrl.hasVType(myManualTypeID)) {
    //         throw ProcessError("The manual vehicle type '" + myManualTypeID +
    //                            "' of vehicle '" + holder.getID() + "' is not known.");
    //     }
    //
    //     // Eventually instantiate given vTypes from distributions
    //     if (myState == ToCState::MANUAL && automatedVTypeIsDist) {
    //         myAutomatedTypeID = vehCtrl.getVType(myAutomatedTypeID, MSRouteHandler::getParsingRNG())->getID();
    //     } else if (myState == ToCState::AUTOMATED && manualVTypeIsDist) {
    //         myManualTypeID = vehCtrl.getVType(myManualTypeID, MSRouteHandler::getParsingRNG())->getID();
    //     }
    //
    //     // register at static instance container
    //     myInstances.insert(this);
    //     initColorScheme();
    //
    // #ifdef DEBUG_TOC
    //     std::cout << "initialized device '" << id << "' with "
    //               << "outputFilename=" << outputFilename << ", "
    //               << "myManualType=" << myManualTypeID << ", "
    //               << "myAutomatedType=" << myAutomatedTypeID << ", "
    //               << "myResponseTime=" << myResponseTime << ", "
    //               << "myRecoveryRate=" << myRecoveryRate << ", "
    //               << "myInitialAwareness=" << myInitialAwareness << ", "
    //               << "myMRMDecel=" << myMRMDecel << ", "
    //               << "ogTimeHeadway=" << myOpenGapParams.newTimeHeadway << ", "
    //               << "ogSpaceHeadway=" << myOpenGapParams.newSpaceHeadway << ", "
    //               << "ogChangeRate=" << myOpenGapParams.changeRate << ", "
    //               << "ogMaxDecel=" << myOpenGapParams.maxDecel << ", "
    //               << "ogActive=" << myOpenGapParams.active << ", "
    //               << "myCurrentAwareness=" << myCurrentAwareness << ", "
    //               << "myState=" << _2string(myState) << std::endl;
    // #endif
    //
    //     assert(myInitialAwareness <= 1.0 && myInitialAwareness >= 0.0);
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, outputFilename: &str, manualType: &str, automatedType: &str, responseTime: SUMOTime, recoveryRate: f64, lcAbstinence: f64, initialAwareness: f64, mrmDecel: f64, dynamicToCThreshold: f64, dynamicMRMProbability: f64, maxPreparationAccel: f64, mrmKeepRight: bool, mrmSafeSpot: &str, mrmSafeSpotDuration: SUMOTime, useColorScheme: bool, ogp: MSDevice_ToC_OpenGapParams) -> Self {
        let base = MSVehicleDevice::new(holder, id);
        let holderPtr = base.myHolder;
        let mut this = MSDevice_ToC {
            base,
            myManualTypeID: manualType.to_string(),
            myAutomatedTypeID: automatedType.to_string(),
            myResponseTime: responseTime,
            myRecoveryRate: recoveryRate,
            myLCAbstinence: lcAbstinence,
            myInitialAwareness: initialAwareness,
            myMRMDecel: mrmDecel,
            myCurrentAwareness: 1.0,
            myColorScheme: BTreeMap::new(),
            myUseColorScheme: useColorScheme,
            myState: MSDevice_ToC_ToCState::UNDEFINED,
            myHolderMS: holderPtr.cast::<MSVehicle>(),
            myTriggerMRMCommand: Ptr::null(),
            myTriggerToCCommand: Ptr::null(),
            myRecoverAwarenessCommand: Ptr::null(),
            myExecuteMRMCommand: Ptr::null(),
            myPrepareToCCommand: Ptr::null(),
            myOutputFile: Ptr::null(),
            myEvents: VecDeque::new(),
            myEventLanes: VecDeque::new(),
            myEventXY: VecDeque::new(),
            myPreviousLCMode: -1,
            myOpenGapParams: ogp,
            myDynamicToCThreshold: dynamicToCThreshold,
            myMRMProbability: dynamicMRMProbability,
            myDynamicToCActive: dynamicToCThreshold > 0.0,
            myIssuedDynamicToC: false,
            myDynamicToCLane: -1,
            myMRMKeepRight: mrmKeepRight,
            myMRMSafeSpot: mrmSafeSpot.to_string(),
            myMRMSafeSpotDuration: mrmSafeSpotDuration,
            myMaxPreparationAccel: maxPreparationAccel,
            myOriginalMaxAccel: -1.0,
        };
        if outputFilename != "" {
            this.myOutputFile = Ptr::from_raw(OutputDevice::get_device(outputFilename, true).unwrap_or_else(|e| panic!("{}", e)).as_ptr());
            // TODO: make xsd, include header
            // myOutputFile.writeXMLHeader("ToCDeviceLog", "ToCDeviceLog.xsd");
            if !CREATED_OUTPUT_FILES.with(|f| f.borrow().contains(outputFilename)) {
                this.myOutputFile.m().write_xml_header("ToCDeviceLog", "", BTreeMap::new(), true);
                CREATED_OUTPUT_FILES.with(|f| f.borrow_mut().insert(outputFilename.to_string()));
            }
        }
        // Check if the given vTypes for the ToC Device are vTypeDistributions
        let netPtr = MSNet::get_instance();
        let vehCtrl = netPtr.m().get_vehicle_control();
        let automatedVTypeIsDist = vehCtrl.has_v_type_distribution(&this.myAutomatedTypeID);
        let manualVTypeIsDist = vehCtrl.has_v_type_distribution(&this.myManualTypeID);
        // Check if the vType of the holder matches one of the given vTypes
        let holderVTypeID = holderPtr.get_vehicle_type().get_id().to_string();
        if holderVTypeID == this.myManualTypeID {
            this.myState = MSDevice_ToC_ToCState::MANUAL;
        } else if holderVTypeID == this.myAutomatedTypeID {
            this.myState = MSDevice_ToC_ToCState::AUTOMATED;
        } else if manualVTypeIsDist && vehCtrl.get_v_type_distribution_membership(&holderVTypeID).contains(&this.myManualTypeID) {
            // Holder type id is from the given manual type distribution.
            this.myState = MSDevice_ToC_ToCState::MANUAL;
            this.myManualTypeID = holderVTypeID.clone();
        } else if automatedVTypeIsDist && vehCtrl.get_v_type_distribution_membership(&holderVTypeID).contains(&this.myAutomatedTypeID) {
            // Holder type id is from the given automated type distribution.
            this.myState = MSDevice_ToC_ToCState::AUTOMATED;
            this.myAutomatedTypeID = holderVTypeID.clone();
        } else {
            panic!("{}", ProcessError::new(format!("Vehicle type of vehicle '{}' ('{}') must coincide with manualType ('{}') or automatedType ('{}') specified for its ToC-device (or drawn from the specified vTypeDistributions).",
                holderPtr.get_id(), holderVTypeID, manualType, automatedType)));
        }
        if !vehCtrl.has_v_type(&this.myAutomatedTypeID) {
            panic!("{}", ProcessError::new(format!("The automated vehicle type '{}' of vehicle '{}' is not known.", this.myAutomatedTypeID, holderPtr.get_id())));
        }
        if !vehCtrl.has_v_type(&this.myManualTypeID) {
            panic!("{}", ProcessError::new(format!("The manual vehicle type '{}' of vehicle '{}' is not known.", this.myManualTypeID, holderPtr.get_id())));
        }
        // Eventually instantiate given vTypes from distributions
        if this.myState == MSDevice_ToC_ToCState::MANUAL && automatedVTypeIsDist {
            let id = this.myAutomatedTypeID.clone();
            this.myAutomatedTypeID = vehCtrl.get_v_type(&id, SUMORouteHandler::get_parsing_rng(), false).get_id().to_string();
        } else if this.myState == MSDevice_ToC_ToCState::AUTOMATED && manualVTypeIsDist {
            let id = this.myManualTypeID.clone();
            this.myManualTypeID = vehCtrl.get_v_type(&id, SUMORouteHandler::get_parsing_rng(), false).get_id().to_string();
        }
        // shortcut: C++ inserts `this` into myInstances here; the Rust constructor returns by value, so the registration is done by the caller (buildVehicleDevices) at the final address
        this.init_color_scheme();
        assert!(this.myInitialAwareness <= 1.0 && this.myInitialAwareness >= 0.0);
        this
    }

    /// C++: `void MSDevice_ToC::initColorScheme()` [private]
    /// Initialize vehicle colors for different states
    // void
    // MSDevice_ToC::initColorScheme() {
    //     //RGBColor(red, green, blue)
    //     myColorScheme[MANUAL] = MSNet::getInstance()->getVehicleControl().getVType(myManualTypeID)->getColor();
    //     myColorScheme[AUTOMATED] = MSNet::getInstance()->getVehicleControl().getVType(myAutomatedTypeID)->getColor();
    //     myColorScheme[PREPARING_TOC] = RGBColor(200, 200, 250); // light blue
    //     myColorScheme[MRM] = RGBColor(250, 50, 50); // red
    //     myColorScheme[RECOVERING] = RGBColor(250, 210, 150); // light yellow
    //     myColorScheme[UNDEFINED] = RGBColor(150, 150, 150); // gray
    // }
    #[allow(unused_variables)]
    pub fn init_color_scheme(&mut self) {
        let netPtr = MSNet::get_instance();
        let vc = netPtr.m().get_vehicle_control();
        let manual = vc.get_v_type(&self.myManualTypeID, SUMORouteHandler::get_parsing_rng(), false).get_color().clone();
        let automated = vc.get_v_type(&self.myAutomatedTypeID, SUMORouteHandler::get_parsing_rng(), false).get_color().clone();
        self.myColorScheme.insert(MSDevice_ToC_ToCState::MANUAL, manual);
        self.myColorScheme.insert(MSDevice_ToC_ToCState::AUTOMATED, automated);
        self.myColorScheme.insert(MSDevice_ToC_ToCState::PREPARING_TOC, RGBColor::new(200, 200, 250, 255)); // light blue
        self.myColorScheme.insert(MSDevice_ToC_ToCState::MRM, RGBColor::new(250, 50, 50, 255)); // red
        self.myColorScheme.insert(MSDevice_ToC_ToCState::RECOVERING, RGBColor::new(250, 210, 150, 255)); // light yellow
        self.myColorScheme.insert(MSDevice_ToC_ToCState::UNDEFINED, RGBColor::new(150, 150, 150, 255)); // gray
    }

    /// C++: `void MSDevice_ToC::setAwareness(double)` [private]
    /// Set the awareness to the given value
    // void
    // MSDevice_ToC::setAwareness(double value) {
    //     if (value > 1.0 || value < 0.0) {
    //         std::stringstream ss;
    //         ss << "Truncating invalid value for awareness (" << value << ") to lie in [0,1].";
    //         WRITE_WARNING(ss.str());
    //         value = MAX2(0.0, MIN2(1.0, value));
    //     }
    //     if (myCurrentAwareness >= myLCAbstinence && value < myLCAbstinence) {
    //         // Awareness is now below LC abstinence level -> prevent deliberate LCs
    //         deactivateDeliberateLCs();
    //     } else if (myCurrentAwareness < myLCAbstinence && value >= myLCAbstinence) {
    //         // Awareness is now above LC abstinence level -> allow deliberate LCs
    //         resetDeliberateLCs();
    //     }
    //     myCurrentAwareness = value;
    //     myHolderMS->getDriverState()->setAwareness(value);
    // }
    #[allow(unused_variables)]
    pub fn set_awareness(&mut self, value: f64) {
        let mut value = value;
        if value > 1.0 || value < 0.0 {
            write_warningf!("Truncating invalid value for awareness (%) to lie in [0,1].", value);
            value = f64::max(0.0, f64::min(1.0, value));
        }
        if self.myCurrentAwareness >= self.myLCAbstinence && value < self.myLCAbstinence {
            // Awareness is now below LC abstinence level -> prevent deliberate LCs
            self.deactivate_deliberate_l_cs();
        } else if self.myCurrentAwareness < self.myLCAbstinence && value >= self.myLCAbstinence {
            // Awareness is now above LC abstinence level -> allow deliberate LCs
            self.reset_deliberate_l_cs();
        }
        self.myCurrentAwareness = value;
        let ds = self.myHolderMS.m().get_driver_state();
        // SAFETY: the driver state is owned by the holder; C++ calls the non-const setter through a pointer
        unsafe { (Rc::as_ptr(&ds) as *mut MSSimpleDriverState).as_mut().unwrap().set_awareness(value) };
    }

    /// C++: `void MSDevice_ToC::setState(ToCState)` [private]
    /// Set the ToC device's state
    // void
    // MSDevice_ToC::setState(ToCState state) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " MSDevice_ToC::setState()" << std::endl;
    // #endif
    //     if (myState == state) {
    //         // No state change
    //         return;
    //     }
    //
    //     if (myState == MRM) {
    //         // reset the vehicle's maxAccel
    //         myHolderMS->getSingularType().getCarFollowModel().setMaxAccel(myOriginalMaxAccel);
    //         resetDeliberateLCs();
    //     } else if (myState == PREPARING_TOC) {
    //         if (myOpenGapParams.active) {
    //             // Deactivate gap control at preparation phase end
    //             myHolderMS->getInfluencer().deactivateGapController();
    //         }
    //         if (state != MRM) {
    //             // Aborting preparation
    //             resetDeliberateLCs();
    //             myHolderMS->getSingularType().getCarFollowModel().setMaxAccel(myOriginalMaxAccel);
    //         }
    //     } else if (state == PREPARING_TOC || state == MRM) {
    // #ifdef DEBUG_TOC
    //         std::cout << "  Entering ToC preparation... " << std::endl;
    // #endif
    //         // Prevent lane changing during takeover preparation
    //         deactivateDeliberateLCs();
    //         // Store original value of maxAccel for restoring it after preparation phase
    //         myOriginalMaxAccel = myHolderMS->getCarFollowModel().getMaxAccel();
    //         // Impose acceleration limit during preparation
    //         myHolderMS->getSingularType().getCarFollowModel().setMaxAccel(MIN2(myMaxPreparationAccel, myOriginalMaxAccel));
    //     }
    //
    //     if (myIssuedDynamicToC) {
    //         // Reset dynamic ToC flag
    //         myIssuedDynamicToC = false;
    //     }
    //
    //     myState = state;
    //     if (myUseColorScheme) {
    //         setVehicleColor();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_state(&mut self, state: MSDevice_ToC_ToCState) {
        if self.myState == state {
            // No state change
            return;
        }
        if self.myState == MSDevice_ToC_ToCState::MRM {
            // reset the vehicle's maxAccel
            let orig = self.myOriginalMaxAccel;
            Ptr::new(self.myHolderMS.m().get_singular_type().get_car_follow_model()).m().set_max_accel(orig);
            self.reset_deliberate_l_cs();
        } else if self.myState == MSDevice_ToC_ToCState::PREPARING_TOC {
            if self.myOpenGapParams.active {
                // Deactivate gap control at preparation phase end
                self.myHolderMS.m().get_influencer().deactivate_gap_controller();
            }
            if state != MSDevice_ToC_ToCState::MRM {
                // Aborting preparation
                self.reset_deliberate_l_cs();
                let orig = self.myOriginalMaxAccel;
                Ptr::new(self.myHolderMS.m().get_singular_type().get_car_follow_model()).m().set_max_accel(orig);
            }
        } else if state == MSDevice_ToC_ToCState::PREPARING_TOC || state == MSDevice_ToC_ToCState::MRM {
            // Prevent lane changing during takeover preparation
            self.deactivate_deliberate_l_cs();
            // Store original value of maxAccel for restoring it after preparation phase
            self.myOriginalMaxAccel = self.myHolderMS.m().get_car_follow_model().get_max_accel();
            // Impose acceleration limit during preparation
            let limit = f64::min(self.myMaxPreparationAccel, self.myOriginalMaxAccel);
            Ptr::new(self.myHolderMS.m().get_singular_type().get_car_follow_model()).m().set_max_accel(limit);
        }
        if self.myIssuedDynamicToC {
            // Reset dynamic ToC flag
            self.myIssuedDynamicToC = false;
        }
        self.myState = state;
        if self.myUseColorScheme {
            self.set_vehicle_color();
        }
    }

    /// C++: `void MSDevice_ToC::setVehicleColor()` [private]
    // void
    // MSDevice_ToC::setVehicleColor() {
    //     const SUMOVehicleParameter& p = myHolder.getParameter();
    //     p.color = myColorScheme[myState];
    //     p.parametersSet |= VEHPARS_COLOR_SET;
    // }
    #[allow(unused_variables)]
    pub fn set_vehicle_color(&mut self) {
        let holderPtr = self.myHolder;
        let color = self.myColorScheme.get(&self.myState).cloned().unwrap();
        // const_cast on the holder parameters, as in C++
        let p: &mut SUMOVehicleParameter = unsafe { &mut *Ptr::new(holderPtr.get_parameter()).as_raw() };
        p.color = color;
        p.parametersSet |= VEHPARS_COLOR_SET;
    }

    /// C++: `void MSDevice_ToC::requestToC(SUMOTime, SUMOTime)` [private]
    /// Request a ToC. If the device is in AUTOMATED or MRM state, a driver response time is sampled and the ToC is scheduled. If the response is larger than timeTillMRM, an MRM is scheduled as well. If the device is in MANUAL or UNDEFINED state, it switches to AUTOMATED. The request is ignored if the state is already PREPARING_TOC.
    // void
    // MSDevice_ToC::requestToC(SUMOTime timeTillMRM, SUMOTime responseTime) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " requestToC() for vehicle '" << myHolder.getID() << "', timeTillMRM=" << timeTillMRM << ", responseTime=" << responseTime << std::endl;
    // #endif
    //     if (myState == AUTOMATED) {
    //         // Initialize preparation phase
    //         if (responseTime == -1000) {
    //             // Sample response time from distribution
    //             const double sample = sampleResponseTime(STEPS2TIME(timeTillMRM));
    //             // this needs to be a separate line because TIME2STEPS may otherwise do two calls to sampleResponseTime
    //             responseTime = TIME2STEPS(sample);
    //         }
    //
    //         // Schedule ToC Event
    //         myTriggerToCCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::triggerDownwardToC);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myTriggerToCCommand, SIMSTEP + responseTime);
    //
    //         assert(myExecuteMRMCommand == nullptr);
    //         assert(myTriggerMRMCommand == nullptr);
    //         if (responseTime > timeTillMRM && myState != MRM) {
    //             // Schedule new MRM if driver response time is higher than permitted
    //             myTriggerMRMCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::triggerMRM);
    //             MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myTriggerMRMCommand, SIMSTEP + timeTillMRM);
    //         }
    //
    //         // Start ToC preparation process
    //         myPrepareToCCommand = new WrappingCommand<MSDevice_ToC>(this, &MSDevice_ToC::ToCPreparationStep);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myPrepareToCCommand, SIMSTEP + DELTA_T);
    //         setState(PREPARING_TOC);
    //         if (myOpenGapParams.active) {
    //             // Start gap controller
    //             double originalTau = myHolderMS->getCarFollowModel().getHeadwayTime();
    //             myHolderMS->getInfluencer().activateGapController(originalTau,
    //                     myOpenGapParams.newTimeHeadway, myOpenGapParams.newSpaceHeadway, -1,
    //                     myOpenGapParams.changeRate, myOpenGapParams.maxDecel);
    //         }
    //         // Record event
    //         if (generatesOutput()) {
    //             myEvents.push(std::make_pair(SIMSTEP, "TOR"));
    //             myEventLanes.push(std::make_pair(myHolder.getLane()->getID(), myHolder.getPositionOnLane())); // add lane and lanepos
    //             myEventXY.push(std::make_pair(myHolder.getPosition().x(), myHolder.getPosition().y()));       // add (x, y) position
    //         }
    //     } else {
    //         // Switch to automated mode is performed immediately
    //         if (timeTillMRM > 0) {
    //             std::stringstream ss;
    //             ss << "[t=" << SIMTIME << "] Positive transition time (" << STEPS2TIME(timeTillMRM) << "s.) for upward ToC of vehicle '" << myHolder.getID() << "' is ignored.";
    //             WRITE_WARNING(ss.str());
    //         }
    //         triggerUpwardToC(SIMSTEP + DELTA_T);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn request_to_c(&mut self, timeTillMRM: SUMOTime, responseTime: SUMOTime) {
        if self.myState == MSDevice_ToC_ToCState::AUTOMATED {
            // Initialize preparation phase
            let mut responseTime = responseTime;
            if responseTime == -1000 {
                // Sample response time from distribution
                let sample = self.sample_response_time(steps2time(timeTillMRM));
                // this needs to be a separate line because TIME2STEPS may otherwise do two calls to sampleResponseTime
                responseTime = time2steps(sample);
            }
            // shortcut: C++ schedules the ToC trigger, MRM trigger and ToC preparation as WrappingCommands; commands are not ported, so nothing is scheduled, upgrade when Command/event control is ported
            assert!(self.myExecuteMRMCommand.is_null());
            assert!(self.myTriggerMRMCommand.is_null());
            self.set_state(MSDevice_ToC_ToCState::PREPARING_TOC);
            if self.myOpenGapParams.active {
                // Start gap controller
                let originalTau = self.myHolderMS.m().get_car_follow_model().get_headway_time();
                let ogp = self.myOpenGapParams.clone();
                self.myHolderMS.m().get_influencer().activate_gap_controller(originalTau,
                    ogp.newTimeHeadway, ogp.newSpaceHeadway, -1.0,
                    ogp.changeRate, ogp.maxDecel, Ptr::null());
            }
            // Record event
            if self.generates_output() {
                self.record_event(MSNet::get_instance().get_current_time_step(), "TOR");
            }
        } else {
            // Switch to automated mode is performed immediately
            if timeTillMRM > 0 {
                write_warningf!("[t=%] Positive transition time (%s.) for upward ToC of vehicle '%' is ignored.",
                    MSNet::get_instance().get_current_time_step(), steps2time(timeTillMRM), self.myHolder.get_id());
            }
            self.trigger_upward_to_c(MSNet::get_instance().get_current_time_step() + delta_t());
        }
    }

    /// C++: `void MSDevice_ToC::requestMRM()` [private]
    /// Request an MRM to be initiated immediately. No downward ToC will be scheduled.
    // void
    // MSDevice_ToC::requestMRM() {
    //     // Remove any preparatory process
    //     descheduleToCPreparation();
    //     // .. and any recovery process
    //     descheduleRecovery();
    //     // ... and any pending ToC to manual
    //     descheduleToC();
    //     // Immediately trigger the MRM process
    //     triggerMRM(0);
    // }
    #[allow(unused_variables)]
    pub fn request_mrm(&mut self) {
        // Remove any preparatory process
        self.deschedule_to_c_preparation();
        // .. and any recovery process
        self.deschedule_recovery();
        // ... and any pending ToC to manual
        self.deschedule_to_c();
        // Immediately trigger the MRM process
        self.trigger_mrm(0);
    }

    /// C++: `void MSDevice_ToC::switchHolderType(const std::string &)` [private]
    /// Switch the device holder's vehicle type
    // void
    // MSDevice_ToC::switchHolderType(const std::string& targetTypeID) {
    // #ifdef DEBUG_TOC
    //     std::cout << SIMTIME << " Switching type of vehicle '" << myHolder.getID() << "' to '" << targetTypeID << "'" << std::endl;
    // #endif
    //     MSVehicleType* targetType = MSNet::getInstance()->getVehicleControl().getVType(targetTypeID);
    //     if (targetType == nullptr) {
    //         WRITE_ERRORF(TL("vType '%' for vehicle '%' is not known."), targetType->getID(), myHolder.getID());
    //         return;
    //     }
    //     myHolderMS->replaceVehicleType(targetType);
    // }
    #[allow(unused_variables)]
    pub fn switch_holder_type(&mut self, targetTypeID: &str) {
        let targetType = MSNet::get_instance().m().get_vehicle_control().get_v_type(targetTypeID, SUMORouteHandler::get_parsing_rng(), false);
        if targetType.is_null() {
            write_errorf!("vType '%' for vehicle '%' is not known.", targetTypeID, self.myHolder.get_id());
            return;
        }
        self.myHolderMS.m().replace_vehicle_type(targetType);
    }

    /// C++: `void MSDevice_ToC::descheduleMRM()` [private]
    /// Break MRM Process or remove MRM-Trigger command from the event-queue.
    // void
    // MSDevice_ToC::descheduleMRM() {
    //     // Eventually abort scheduled MRM
    //     if (myTriggerMRMCommand != nullptr) {
    //         myTriggerMRMCommand->deschedule();
    //         myTriggerMRMCommand = nullptr;
    //     }
    //     // Eventually abort ongoing MRM
    //     if (myExecuteMRMCommand != nullptr) {
    //         myExecuteMRMCommand->deschedule();
    //         resetDeliberateLCs();
    //         myExecuteMRMCommand = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule_mrm(&mut self) {
        // Eventually abort scheduled MRM
        if !self.myTriggerMRMCommand.is_null() {
            // shortcut: deschedule() of the WrappingCommand is not ported, upgrade when Command/event control is ported
            self.myTriggerMRMCommand = Ptr::null();
        }
        // Eventually abort ongoing MRM
        if !self.myExecuteMRMCommand.is_null() {
            // shortcut: deschedule() of the WrappingCommand is not ported, upgrade when Command/event control is ported
            self.reset_deliberate_l_cs();
            self.myExecuteMRMCommand = Ptr::null();
        }
    }

    /// C++: `void MSDevice_ToC::descheduleToC()` [private]
    /// Remove scheduled ToC-Trigger command from the event-queue.
    // void
    // MSDevice_ToC::descheduleToC() {
    //     if (myTriggerToCCommand != nullptr) {
    //         myTriggerToCCommand->deschedule();
    //         myTriggerToCCommand = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule_to_c(&mut self) {
        if !self.myTriggerToCCommand.is_null() {
            // shortcut: deschedule() of the WrappingCommand is not ported, upgrade when Command/event control is ported
            self.myTriggerToCCommand = Ptr::null();
        }
    }

    /// C++: `void MSDevice_ToC::descheduleToCPreparation()` [private]
    /// Remove ongoing ToC-Preparation process from the event-queue.
    // void
    // MSDevice_ToC::descheduleToCPreparation() {
    //     // Eventually stop ToC preparation process
    //     if (myPrepareToCCommand != nullptr) {
    //         myPrepareToCCommand->deschedule();
    //         if (myState != MRM) {
    //             resetDeliberateLCs();
    //         }
    //         myPrepareToCCommand = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule_to_c_preparation(&mut self) {
        // Eventually stop ToC preparation process
        if !self.myPrepareToCCommand.is_null() {
            // shortcut: deschedule() of the WrappingCommand is not ported, upgrade when Command/event control is ported
            if self.myState != MSDevice_ToC_ToCState::MRM {
                self.reset_deliberate_l_cs();
            }
            self.myPrepareToCCommand = Ptr::null();
        }
    }

    /// C++: `void MSDevice_ToC::descheduleRecovery()` [private]
    /// Remove ongoing awareness recovery process from the event-queue.
    // void
    // MSDevice_ToC::descheduleRecovery() {
    //     // Eventually stop ToC preparation process
    //     if (myRecoverAwarenessCommand != nullptr) {
    //         myRecoverAwarenessCommand->deschedule();
    //         myRecoverAwarenessCommand = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deschedule_recovery(&mut self) {
        // Eventually stop ToC preparation process
        if !self.myRecoverAwarenessCommand.is_null() {
            // shortcut: deschedule() of the WrappingCommand is not ported, upgrade when Command/event control is ported
            self.myRecoverAwarenessCommand = Ptr::null();
        }
    }

    /// C++: `void MSDevice_ToC::resetDeliberateLCs()` [private]
    /// Resets the holder's LC mode to the last differing to LCModeMRM
    // void
    // MSDevice_ToC::resetDeliberateLCs() {
    //     if (myPreviousLCMode != -1) {
    //         myHolderMS->getInfluencer().setLaneChangeMode(myPreviousLCMode);
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " MSDevice_ToC::resetLCMode() restoring LC Mode of vehicle '" << myHolder.getID() << "' to " << myPreviousLCMode << std::endl;
    // #endif
    //     }
    //     myPreviousLCMode = -1;
    // }
    #[allow(unused_variables)]
    pub fn reset_deliberate_l_cs(&mut self) {
        if self.myPreviousLCMode != -1 {
            let mode = self.myPreviousLCMode;
            self.myHolderMS.m().get_influencer().set_lane_change_mode(mode);
        }
        self.myPreviousLCMode = -1;
    }

    /// C++: `void MSDevice_ToC::deactivateDeliberateLCs()` [private]
    /// Resets the holder's LC mode to the operational LC-mode of the ToC Device (
    // void
    // MSDevice_ToC::deactivateDeliberateLCs() {
    //     const int lcModeHolder = myHolderMS->getInfluencer().getLaneChangeMode();
    //     if (lcModeHolder != LCModeMRM) {
    //         myPreviousLCMode = lcModeHolder;
    // #ifdef DEBUG_TOC
    //         std::cout << SIMTIME << " MSDevice_ToC::setLCModeMRM() setting LC Mode of vehicle '" << myHolder.getID()
    //                   << "' from " << myPreviousLCMode << " to " << LCModeMRM << std::endl;
    // #endif
    //         myHolderMS->getInfluencer().setLaneChangeMode(LCModeMRM);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deactivate_deliberate_l_cs(&mut self) {
        let lcModeHolder = self.myHolderMS.m().get_influencer().get_lane_change_mode();
        if lcModeHolder != LCModeMRM {
            self.myPreviousLCMode = lcModeHolder;
            self.myHolderMS.m().get_influencer().set_lane_change_mode(LCModeMRM);
        }
    }

    /// C++: `bool MSDevice_ToC::isManuallyDriven()` [private]
    /// Whether the current operation mode is manual
    // bool
    // MSDevice_ToC::isManuallyDriven() {
    //     return (myState == MANUAL || myState == RECOVERING);
    // }
    #[allow(unused_variables)]
    pub fn is_manually_driven(&mut self) -> bool {
        self.myState == MSDevice_ToC_ToCState::MANUAL || self.myState == MSDevice_ToC_ToCState::RECOVERING
    }

    /// C++: `bool MSDevice_ToC::isAutomated()` [private]
    /// Whether the current operation mode is automated
    // bool
    // MSDevice_ToC::isAutomated() {
    //     return (myState == AUTOMATED || myState == PREPARING_TOC || myState == MRM);
    // }
    #[allow(unused_variables)]
    pub fn is_automated(&mut self) -> bool {
        self.myState == MSDevice_ToC_ToCState::AUTOMATED || self.myState == MSDevice_ToC_ToCState::PREPARING_TOC || self.myState == MSDevice_ToC_ToCState::MRM
    }

    /// C++: `bool MSDevice_ToC::checkDynamicToC()` [private]
    /// Check if the vehicle should induce a ToC due to internal reasons. That is, if the route cannot be followed for more time than a given threshold,
    // bool
    // MSDevice_ToC::checkDynamicToC() {
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << SIMTIME << " # MSDevice_ToC::checkDynamicToC() for veh '" << myHolder.getID() << "'" << std::endl;
    // #endif
    //     if (!myDynamicToCActive) {
    //         return false;
    //     }
    //     // The vehicle's current lane
    //     const MSLane* currentLane = myHolderMS->getLane();
    //
    //     if (currentLane->isInternal()) {
    //         // Don't start or abort dynamic ToCs on internal lanes
    //         return myIssuedDynamicToC;
    //     }
    //
    //     if (myIssuedDynamicToC) {
    // #ifdef DEBUG_DYNAMIC_TOC
    //         std::cout << SIMTIME << " Dynamic ToC is ongoing." << std::endl;
    // #endif
    //         // Dynamic ToC in progress. Resist to aborting it if lane was not changed.
    //         if (myDynamicToCLane == currentLane->getNumericalID()) {
    //             return true;
    //         }
    //     }
    //     // Length for which the current route can be followed
    //     const std::vector<MSVehicle::LaneQ>& bestLanes = myHolderMS->getBestLanes();
    //     // Maximal distance for route continuation without LCs over the possible start lanes
    //     double maximalContinuationDistance = 0;
    //     // Distance for route continuation without LCs from the vehicle's current lane
    //     double continuationDistanceOnCurrent = 0;
    //     // Lane of the next stop
    //     const MSLane* nextStopLane = nullptr;
    //
    //     if (myHolderMS->hasStops()) {
    //         nextStopLane = myHolderMS->getNextStop().lane;
    //     }
    //     for (auto& i : bestLanes) {
    //         maximalContinuationDistance = MAX2(maximalContinuationDistance, i.length);
    //         if (currentLane == i.lane) {
    //             if (myHolderMS->hasStops()) {
    //                 // Check if the next stop lies on the route continuation from the current lane
    //                 for (MSLane* l : i.bestContinuations) {
    //                     if (l == nextStopLane) {
    // #ifdef DEBUG_DYNAMIC_TOC
    //                         std::cout << SIMTIME << " Stop found on the route continuation from the current lane. => No ToC" << std::endl;
    // #endif
    //                         // Stop found on the route continuation from the current lane => no ToC necessary
    //                         return false;
    //                     }
    //                 }
    //             }
    //             continuationDistanceOnCurrent = i.length;
    //         }
    //     }
    //     if (continuationDistanceOnCurrent == maximalContinuationDistance) {
    //         // There is no better lane than the current, hence no desire to change lanes,
    //         // which the driver could pursue better than the automation => no reason for ToC.
    //         return false;
    //     }
    //     const double distFromCurrent = continuationDistanceOnCurrent - myHolderMS->getPositionOnLane();
    //     const double MRMDist = 0.5 * myHolderMS->getSpeed() * myHolderMS->getSpeed() / MAX2(myMRMDecel, 0.0001);
    //     double distThreshold = myHolderMS->getSpeed() * myDynamicToCThreshold + MRMDist;
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << "  speed=" << myHolderMS->getSpeed()
    //               << ", distFromCurrent=" << distFromCurrent
    //               << ", maximal dist=" << maximalContinuationDistance - myHolderMS->getPositionOnLane()
    //               << ", distThreshold=" << distThreshold
    //               << std::endl;
    // #endif
    //
    //     if (myIssuedDynamicToC) {
    //         // In case of an ongoing ToC, add an additional resistance to abort it.
    //         // (The lane-check above does not capture lanes subsequent to the dynamic ToC lane)
    //         distThreshold *= DYNAMIC_TOC_ABORT_RESISTANCE_FACTOR;
    //     }
    //
    //     if (distFromCurrent < distThreshold) {
    //         // TODO: Make this more sophisticated in dealing with low speeds/stops and route ends
    // #ifdef DEBUG_DYNAMIC_TOC
    //         std::cout << SIMTIME << "  * distAlongBest is below threshold! *" << std::endl;
    // #endif
    //         return true;
    //     }
    //
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_dynamic_to_c(&mut self) -> bool {
        if !self.myDynamicToCActive {
            return false;
        }
        // The vehicle's current lane
        let holderPtr = self.myHolderMS;
        let currentLane = holderPtr.get_lane();
        if currentLane.is_internal() {
            // Don't start or abort dynamic ToCs on internal lanes
            return self.myIssuedDynamicToC;
        }
        if self.myIssuedDynamicToC {
            // Dynamic ToC in progress. Resist to aborting it if lane was not changed.
            if self.myDynamicToCLane == currentLane.get_numerical_id() {
                return true;
            }
        }
        // Length for which the current route can be followed
        let bestLanes = holderPtr.get_best_lanes();
        // Maximal distance for route continuation without LCs over the possible start lanes
        let mut maximalContinuationDistance: f64 = 0.0;
        // Distance for route continuation without LCs from the vehicle's current lane
        let mut continuationDistanceOnCurrent: f64 = 0.0;
        // Lane of the next stop
        let mut nextStopLane: Ptr<MSLane> = Ptr::null();
        if holderPtr.has_stops() {
            nextStopLane = holderPtr.get_next_stop().lane;
        }
        for i in bestLanes.iter() {
            maximalContinuationDistance = f64::max(maximalContinuationDistance, i.length);
            if currentLane == i.lane {
                if holderPtr.has_stops() {
                    // Check if the next stop lies on the route continuation from the current lane
                    for &l in i.bestContinuations.iter() {
                        if l == nextStopLane {
                            // Stop found on the route continuation from the current lane => no ToC necessary
                            return false;
                        }
                    }
                }
                continuationDistanceOnCurrent = i.length;
            }
        }
        if continuationDistanceOnCurrent == maximalContinuationDistance {
            // There is no better lane than the current, hence no desire to change lanes,
            // which the driver could pursue better than the automation => no reason for ToC.
            return false;
        }
        let distFromCurrent = continuationDistanceOnCurrent - holderPtr.get_position_on_lane();
        let MRMDist = 0.5 * holderPtr.get_speed() * holderPtr.get_speed() / f64::max(self.myMRMDecel, 0.0001);
        let mut distThreshold = holderPtr.get_speed() * self.myDynamicToCThreshold + MRMDist;
        if self.myIssuedDynamicToC {
            // In case of an ongoing ToC, add an additional resistance to abort it.
            // (The lane-check above does not capture lanes subsequent to the dynamic ToC lane)
            distThreshold *= DYNAMIC_TOC_ABORT_RESISTANCE_FACTOR;
        }
        if distFromCurrent < distThreshold {
            // TODO: Make this more sophisticated in dealing with low speeds/stops and route ends
            return true;
        }
        false
    }

    /// C++: `double MSDevice_ToC::responseTimeMean(double)` [static, private]
    /// Mean of the response time distribution. (Only depends on given lead time)
    // static double responseTimeMean(double leadTime) {
    //         return MIN2(2 * sqrt(leadTime), 0.7 * leadTime);
    //     }
    #[allow(unused_variables)]
    pub fn response_time_mean(leadTime: f64) -> f64 {
        f64::min(2.0 * leadTime.sqrt(), 0.7 * leadTime)
    }

    /// C++: `double MSDevice_ToC::sampleResponseTime(double) const` [const, private]
    /// Samples a random driver response time from a truncated Gaussian with parameters according to the lookup tables
    // double
    // MSDevice_ToC::sampleResponseTime(double leadTime) const {
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << "sampleResponseTime() leadTime=" << leadTime << std::endl;
    // #endif
    //     const double mean = responseTimeMean(leadTime);
    //     const double var = interpolateVariance(leadTime, myMRMProbability);
    //     double rt = RandHelper::randNorm(mean, var, &myResponseTimeRNG);
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << "  mean=" << mean << ", variance=" << var << " => sampled responseTime=" << rt << std::endl;
    // #endif
    //     int it_count = 0;
    //     while (rt < 0 && it_count < MAX_RESPONSETIME_SAMPLE_TRIES) {
    //         rt = RandHelper::randNorm(mean, var, &myResponseTimeRNG);
    //         it_count++;
    //     }
    //     if (rt < 0) {
    //         // Didn't generate a positive random response time => use mean
    //         rt = mean;
    //     }
    //     return rt;
    // }
    #[allow(unused_variables)]
    pub fn sample_response_time(&self, leadTime: f64) -> f64 {
        let mean = Self::response_time_mean(leadTime);
        let var = Self::interpolate_variance(leadTime, self.myMRMProbability);
        let rng = MY_RESPONSE_TIME_RNG.with(|r| Ptr::from_mut(unsafe { &mut *r.as_ptr() }));
        let mut rt = rng.m().rand_norm(mean, var);
        let mut it_count = 0;
        while rt < 0.0 && it_count < MAX_RESPONSETIME_SAMPLE_TRIES {
            rt = rng.m().rand_norm(mean, var);
            it_count += 1;
        }
        if rt < 0.0 {
            // Didn't generate a positive random response time => use mean
            rt = mean;
        }
        rt
    }

    /// C++: `double MSDevice_ToC::interpolateVariance(double, double)` [static, private]
    /// Two-dimensional interpolation of variance from lookup table assumes pMRM >= 0, leadTime >= 0
    // double
    // MSDevice_ToC::interpolateVariance(double leadTime, double pMRM) {
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << "interpolateVariance() leadTime=" << leadTime << ", pMRM=" << pMRM << std::endl;
    // #endif
    //     // Calculate indices for surrounding values in lookup tables
    //
    //     // Find largest p_{i-1} < pMRM < p_{i}
    //     const auto pi = std::lower_bound(lookupResponseTimeMRMProbs.begin(), lookupResponseTimeMRMProbs.end(), pMRM);
    //     if (pi == lookupResponseTimeMRMProbs.end()) {
    //         // requested probability lies outside lookup table.
    //         // => return maximal variance value
    //         return MAX_RESPONSETIME_VARIANCE;
    //     }
    //     const size_t pi1 = pi - lookupResponseTimeMRMProbs.begin();
    //     assert(pi1 > 0);
    //     const size_t pi0 = pi1 - 1;
    //     const double cp = (pMRM - * (pi - 1)) / (*pi - * (pi - 1));
    //
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << " p[=" << pi0 << "]=" << *(pi - 1) << ", p[=" << pi1 << "]=" << *pi << " => cp=" << cp << std::endl;
    // #endif
    //
    //     // Find largest p_{i-1} < pMRM < p_{i}
    //     auto li = std::lower_bound(lookupResponseTimeLeadTimes.begin(), lookupResponseTimeLeadTimes.end(), leadTime);
    //     if (li == lookupResponseTimeLeadTimes.begin()) {
    //         // Given lead time smaller than minimal lookup-value.
    //         // Use minimal value from lookup table instead
    //         leadTime = *li;
    //         li = lookupResponseTimeLeadTimes.begin() + 1;
    //     } else if (li == lookupResponseTimeLeadTimes.end()) {
    //         // Given leadTime exceeds values in lookup table
    //         // => induce extrapolation
    //         li--;
    //     }
    //     const size_t li1 = li - lookupResponseTimeLeadTimes.begin();
    //     const size_t li0 = li1 - 1;
    //     const double cl = (leadTime - * (li - 1)) / (*li - * (li - 1));
    //
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << " l[=" << li0 << "]=" << *(li - 1) << ", l[=" << li1 << "]=" << *li << " => cp=" << cl << std::endl;
    // #endif
    //
    //     // 2D interpolation for variance
    //     // First, interpolate (or extrapolate) variances along leadTimes
    //     const double var00 = lookupResponseTimeVariances[pi0][li0];
    //     const double var01 = lookupResponseTimeVariances[pi0][li1];
    //     const double var10 = lookupResponseTimeVariances[pi1][li0];
    //     const double var11 = lookupResponseTimeVariances[pi1][li1];
    //     const double var_0 = var00 + (var01 - var00) * cl;
    //     const double var_1 = var10 + (var11 - var10) * cl;
    //     // From these, interpolate along the pMRM-axis
    //     const double var = var_0 + (var_1 - var_0) * cp;
    // #ifdef DEBUG_DYNAMIC_TOC
    //     std::cout << " var00=" << var00 << ", var01=" << var01 << " var10=" << var10 << ", var11=" << var11
    //               << " var_0=" << var_0 << ", var_1=" << var_1 << ", var=" << var << std::endl;
    // #endif
    //     return var;
    // }
    #[allow(unused_variables)]
    pub fn interpolate_variance(mut leadTime: f64, pMRM: f64) -> f64 {
        // Calculate indices for surrounding values in lookup tables
        // Find largest p_{i-1} < pMRM < p_{i}
        let pi = match LOOKUP_RESPONSE_TIME_MRM_PROBS.iter().position(|&p| p >= pMRM) {
            Some(i) => i,
            None => {
                // requested probability lies outside lookup table.
                // => return maximal variance value
                return MAX_RESPONSETIME_VARIANCE;
            }
        };
        let pi1 = pi;
        assert!(pi1 > 0);
        let pi0 = pi1 - 1;
        let cp = (pMRM - LOOKUP_RESPONSE_TIME_MRM_PROBS[pi - 1]) / (LOOKUP_RESPONSE_TIME_MRM_PROBS[pi] - LOOKUP_RESPONSE_TIME_MRM_PROBS[pi - 1]);
        // Find largest p_{i-1} < pMRM < p_{i}
        let lt = LOOKUP_RESPONSE_TIME_LEAD_TIMES;
        let mut li = lt.iter().position(|&l| l >= leadTime).unwrap_or(lt.len());
        if li == 0 {
            // Given lead time smaller than minimal lookup-value.
            // Use minimal value from lookup table instead
            leadTime = lt[0];
            li = 1;
        } else if li == lt.len() {
            // Given leadTime exceeds values in lookup table
            // => induce extrapolation
            li -= 1;
        }
        let li1 = li;
        let li0 = li1 - 1;
        let cl = (leadTime - lt[li - 1]) / (lt[li] - lt[li - 1]);
        // 2D interpolation for variance
        // First, interpolate (or extrapolate) variances along leadTimes
        let var00 = LOOKUP_RESPONSE_TIME_VARIANCES[pi0][li0];
        let var01 = LOOKUP_RESPONSE_TIME_VARIANCES[pi0][li1];
        let var10 = LOOKUP_RESPONSE_TIME_VARIANCES[pi1][li0];
        let var11 = LOOKUP_RESPONSE_TIME_VARIANCES[pi1][li1];
        let var_0 = var00 + (var01 - var00) * cl;
        let var_1 = var10 + (var11 - var10) * cl;
        // From these, interpolate along the pMRM-axis
        let var = var_0 + (var_1 - var_0) * cp;
        var
    }

    /// C++: `void MSDevice_ToC::MSDevice_ToC(const MSDevice_ToC &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_ToC) -> Self {
        panic!("MSDevice_ToC copy constructor is private and undefined in C++")
    }

    /// C++: `MSDevice_ToC & MSDevice_ToC::operator=(const MSDevice_ToC &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_ToC) -> &mut MSDevice_ToC {
        panic!("MSDevice_ToC assignment operator is private and undefined in C++")
    }

}

/// C++ class `MSDevice_ToC::OpenGapParams` (microsim/devices/MSDevice_ToC.h)
#[derive(Clone, Copy)]
pub struct MSDevice_ToC_OpenGapParams {
    pub newTimeHeadway: f64,
    pub newSpaceHeadway: f64,
    pub changeRate: f64,
    pub maxDecel: f64,
    pub active: bool,
}

impl MSDevice_ToC_OpenGapParams {
    /// C++: `void OpenGapParams::OpenGapParams(double, double, double, double, bool)`
    // OpenGapParams(double timegap, double spacing, double changeRate, double maxDecel, bool active) :
    //             newTimeHeadway(timegap), newSpaceHeadway(spacing), changeRate(changeRate), maxDecel(maxDecel), active(active)
    //         {}
    #[allow(unused_variables)]
    pub fn new(timegap: f64, spacing: f64, changeRate: f64, maxDecel: f64, active: bool) -> Self {
        MSDevice_ToC_OpenGapParams {
            newTimeHeadway: timegap,
            newSpaceHeadway: spacing,
            changeRate,
            maxDecel,
            active,
        }
    }

}

