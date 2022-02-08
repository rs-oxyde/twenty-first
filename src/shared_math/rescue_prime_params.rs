use crate::shared_math::rescue_prime::RescuePrime;

use super::b_field_element::BFieldElement;

pub fn rescue_prime_params_bfield_0() -> RescuePrime {
    let mds: Vec<Vec<BFieldElement>> = to_matrix(vec![
        vec![
            5910257123858819639,
            3449115226714951713,
            16770055338049327985,
            610399731775780810,
            7363016345531076300,
            16174724756564259629,
            8736587794472183152,
            12699016954477470956,
            13948112026909862966,
            18015813124076612987,
            9568929147539067610,
            14859461777592116402,
            18169364738825153183,
            18221568702798258352,
            1524268296724555606,
            5538821761600,
        ],
        vec![
            1649528676200182784,
            336497118937017052,
            15805000027048028625,
            15709375513998678646,
            14837031240173858084,
            11366298206428370494,
            15698532768527519720,
            5911577595727321095,
            16676030327621016157,
            16537624251746851423,
            13325141695736654367,
            9337952653454313447,
            9090375522091353302,
            5605636660979522224,
            6357222834896114791,
            7776871531164456679,
        ],
        vec![
            8264739868177574620,
            12732288338686680125,
            13022293791945187811,
            17403057736098613442,
            2871266924987061743,
            13286707530570640459,
            9229362695439112266,
            815317759014579856,
            7447771153889267897,
            2209002535000750347,
            3280506473249596174,
            13756142018694965622,
            10518080861296830621,
            16578355848983066277,
            12732532221704648123,
            3426526797578099186,
        ],
        vec![
            8563516248221808333,
            13079317959606236131,
            15645458946300428515,
            9958819147895829140,
            13028053188247480206,
            6789511720078828478,
            6583246594815170294,
            4423695887326249884,
            9751139665897711642,
            10039202025292797758,
            12208726994829996150,
            6238795140281096003,
            9113696057226188857,
            9898705245385052191,
            4213712701625520075,
            8038355032286280912,
        ],
        vec![
            426685147605824917,
            7673465577918025498,
            8452867379070564008,
            10827610229277395180,
            16155539332955658546,
            1575428636717115288,
            8765972548498757598,
            8405996249707890526,
            14855028677418679455,
            17878170012428694685,
            16572621079016066883,
            5311046098447994501,
            10635376800783355348,
            14205668690430323921,
            1181422971831412672,
            4651053123208915543,
        ],
        vec![
            12465667489477238576,
            7300129031676503132,
            13458544786180633209,
            8946801771555977477,
            14203890406114400141,
            8219081892380458635,
            6035067543134909245,
            15140374581570897616,
            4514006299509426029,
            16757530089801321524,
            13202061911440346802,
            11227558237427129334,
            315998614524336401,
            11280705904396606227,
            5798516367202621128,
            17154761698338453414,
        ],
        vec![
            13574436947400004837,
            3126509266905053998,
            10740979484255925394,
            9273322683773825324,
            15349096509718845737,
            14694022445619674948,
            8733857890739087596,
            3198488337424282101,
            9521016570828679381,
            11267736037298472148,
            14825280481028844943,
            1326588754335738002,
            6200834522767914499,
            1070210996042416038,
            9140190343656907671,
            15531381283521001952,
        ],
        vec![
            253143295675927354,
            11977331414401291539,
            13941376566367813256,
            469904915148256197,
            10873951860155749104,
            3939719938926157877,
            2271392376641547055,
            4725974756185387075,
            14827835543640648161,
            17663273767033351157,
            12440960700789890843,
            16589620022628590428,
            12838889473653138505,
            11170336581460183657,
            7583333056198317221,
            6006908286410425140,
        ],
        vec![
            15648567098514276013,
            188901633101859949,
            12256163716419861419,
            17319784688409668747,
            9648971065289440425,
            11370683735445551679,
            11265203235776280908,
            1737672785338087677,
            5225587291780939578,
            4739055740469849012,
            1212344601223444182,
            12958616893209019599,
            7922060480554370635,
            14661420107595710445,
            11744359917257111592,
            9674559564931202709,
        ],
        vec![
            8326110231976411065,
            16856751238353701757,
            7515652322254196544,
            2062531989536141174,
            3875321171362100965,
            1164854003752487518,
            3997098993859160292,
            4074090397542250057,
            3050858158567944540,
            4568245569065883863,
            14559440781022773799,
            5401845794552358815,
            6544584366002554176,
            2511522072283652847,
            9759884967674698659,
            16411672358681189856,
        ],
        vec![
            11392578809073737776,
            8013631514034873271,
            11439549174997471674,
            6373021446442411366,
            12491600135569477757,
            1017093281401495736,
            663547836518863091,
            16157302719777897692,
            11208801522915446640,
            10058178191286215107,
            5521712058210208094,
            3611681474253815005,
            4864578569041337696,
            12270319000993569289,
            7347066511426336318,
            6696546239958933736,
        ],
        vec![
            3335469193383486908,
            12719366334180058014,
            14123019207894489639,
            11418186023060178542,
            2042199956854124583,
            17539253100488345226,
            16240833881391672847,
            11712520063241304909,
            6456900719511754234,
            1819022137223501306,
            7371152900053879920,
            6521878675261223812,
            2050999666988944811,
            8262038465464898064,
            13303819303390508091,
            12657292926928303663,
        ],
        vec![
            8794128680724662595,
            4068577832515945116,
            758247715040138478,
            5600369601992438532,
            3369463178350382224,
            13763645328734311418,
            9685701761982837416,
            2711119809520557835,
            11680482056777716424,
            10958223503056770518,
            4168390070510137163,
            10823375744683484459,
            5613197991565754677,
            11781942063118564684,
            9352512500813609723,
            15997830646514778986,
        ],
        vec![
            7407352006524266457,
            15312663387608602775,
            3026364159907661789,
            5698531403379362946,
            2544271242593770624,
            13104502948897878458,
            7840062700088318710,
            6028743588538970215,
            6144415809411296980,
            468368941216390216,
            3638618405705274008,
            11105401941482704573,
            1850274872877725129,
            1011155312563349004,
            3234620948537841909,
            3818372677739507813,
        ],
        vec![
            4863130691592118581,
            8942166964590283171,
            3639677194051371072,
            15477372418124081864,
            10322228711752830209,
            9139111778956611066,
            202171733050704358,
            11982413146686512577,
            11001000478006340870,
            5491471715020327065,
            6969114856449768266,
            11088492421847219924,
            12913509272810999025,
            17366506887360149369,
            7036328554328346102,
            11139255730689011050,
        ],
        vec![
            2844974929907956457,
            6488525141985913483,
            2860098796699131680,
            10366343151884073105,
            844875652557703984,
            1053177270393416978,
            5189466196833763142,
            1024738234713107670,
            8846741799369572841,
            14490406830213564822,
            10577371742628912722,
            3276210642025060502,
            2605621719516949928,
            5417148926702080639,
            11100652475866543814,
            5247366835775169839,
        ],
    ]);

    let mds_inv: Vec<Vec<BFieldElement>> = to_matrix(vec![
        vec![
            1572742562154761373,
            11904188991461183391,
            16702037635100780588,
            10395027733616703929,
            8130016957979279389,
            12091057987196709719,
            14570460902390750822,
            13452497170858892918,
            7302470671584418296,
            12930709087691977410,
            6940810864055149191,
            15479085069460687984,
            15273989414499187903,
            8742532579937987008,
            78143684950290654,
            10454925311792498315,
        ],
        vec![
            7789818152192856725,
            3486011543032592030,
            17188770042768805161,
            10490412495468775616,
            298640180115056798,
            12895819509602002088,
            1755013598313843104,
            17242416429764373372,
            993835663551930043,
            17604339535769584753,
            17954116481891390155,
            332811330083846624,
            14730023810555747819,
            435413210797820565,
            1781261080337413422,
            4148505421656051973,
        ],
        vec![
            980199695323775177,
            4706730905557535223,
            12734714246714791746,
            14273996233795959868,
            7921735635146743134,
            14772166129594741813,
            2171393332099124215,
            11431591906353698662,
            1968460689143086961,
            12435956952300281356,
            18203712123938736914,
            13226878153002754824,
            4722189513468037980,
            14552059159516237140,
            2186026037853355566,
            11286141841507813990,
        ],
        vec![
            565856028734827369,
            13655906686104936396,
            8559867348362880285,
            2797343365604350633,
            4465794635391355875,
            10602340776590577912,
            6532765362293732644,
            9971594382705594993,
            8246981798349136173,
            4260734168634971109,
            3096607081570771,
            823237991393038853,
            17532689952600815755,
            12134755733102166916,
            10570439735096051664,
            18403803913856082900,
        ],
        vec![
            13128404168847275462,
            16663835358650929116,
            16546671721888068220,
            4685011688485137218,
            1959001578540316019,
            16340711608595843821,
            9460495021221259854,
            3858517940845573321,
            9427670160758976948,
            18064975260450261693,
            4905506444249847758,
            15986418616213903133,
            9282818778268010424,
            9769107232941785010,
            8521948467436343364,
            7419602577337727529,
        ],
        vec![
            5926710664024036226,
            11667040483862285999,
            12291037072726747355,
            12257844845576909578,
            5216888292865522221,
            4949589496388892504,
            6571373688631618567,
            10091372984903831417,
            6240610640427541397,
            6328690792776976228,
            11836184983048970818,
            12710419323566440454,
            10374451385652807364,
            8254232795575550118,
            9866490979395302091,
            12991014125893242232,
        ],
        vec![
            1063347186953727863,
            2952135743830082310,
            17315974856538709017,
            14554512349953922358,
            14134347382797855179,
            17882046380988406016,
            17463193400175360824,
            3726957756828900632,
            17604631050958608669,
            7585987025945897953,
            14470977033142357695,
            10643295498661723800,
            8871197056529643534,
            8384208064507509379,
            9280566467635869786,
            87319369282683875,
        ],
        vec![
            1100172740622998121,
            622721254307916221,
            16843330035110191506,
            13024130485811341782,
            12334996107415540952,
            461552745543935046,
            8140793910765831499,
            9008477689109468885,
            17409910369122253035,
            1804565454784197696,
            5310948951638903141,
            12531953612536647976,
            6147853502869470889,
            1125351356112285953,
            6467901683012265601,
            16792548587138841945,
        ],
        vec![
            14092833521360698433,
            13651748079341829335,
            10688258556205752814,
            1823953496327460008,
            2558053704584850519,
            13269131806718310421,
            4608410977522599149,
            9221187654763620553,
            4611978991500182874,
            8855429001286425455,
            5696709580182222832,
            17579496245625003067,
            5267934104348282564,
            1835676094870249003,
            3542280417783105151,
            11824126253481498070,
        ],
        vec![
            9504622962336320170,
            17887320494921151801,
            6574518722274623914,
            16658124633332643846,
            13808019273382263890,
            13092903038683672100,
            501471167473345282,
            11161560208140424921,
            13001827442679699140,
            14739684132127818993,
            2868223407847949089,
            1726410909424820290,
            6794531346610991076,
            6698331109000773276,
            3680934785728193940,
            8875468921351982841,
        ],
        vec![
            5477651765997654015,
            12280771278642823764,
            3619998794343148112,
            6883119128428826230,
            13512760119042878827,
            3675597821767844913,
            5414638790278102151,
            3587251244316549755,
            17100313981528550060,
            11048426899172804713,
            1396562484529002856,
            2252873797267794672,
            14201526079271439737,
            16618356769072634008,
            144564843743666734,
            11912794688498369701,
        ],
        vec![
            10937102025343594422,
            15432144252435329607,
            2221546737981282133,
            6015808993571140081,
            7447996510907844453,
            7039231904611782781,
            2218118803134364409,
            9472427559993341443,
            11066826455107746221,
            6223571389973384864,
            13615228926415811268,
            10241352486499609335,
            12605380114102527595,
            11403123666082872720,
            9771232158486004346,
            11862860570670038891,
        ],
        vec![
            10489319728736503343,
            588166220336712628,
            524399652036013851,
            2215268375273320892,
            1424724725807107497,
            2223952838426612865,
            1901666565705039600,
            14666084855112001547,
            16529527081633002035,
            3475787534446449190,
            17395838083455569055,
            10036301139275236437,
            5830062976180250577,
            6201110308815839738,
            3908827014617539568,
            13269427316630307104,
        ],
        vec![
            1104974093011983663,
            335137437077264843,
            13411663683768112565,
            7907493007733959147,
            17240291213488173803,
            6357405277112016289,
            7875258449007392338,
            16100900298327085499,
            13542432207857463387,
            9466802464896264825,
            9221606791343926561,
            10417300838622453849,
            13201838829839066427,
            9833345239958202067,
            16688814355354359676,
            13315432437333533951,
        ],
        vec![
            378443609734580293,
            14654525144709164243,
            1967217494445269914,
            16045947041840686058,
            18049263629128746044,
            1957063364541610677,
            16123386013589472221,
            5923137592664329389,
            12399617421793397670,
            3403518680407886401,
            6416516714555000604,
            13286977196258324106,
            17641011370212535641,
            14823578540420219384,
            11909888788340877523,
            11040604022089158722,
        ],
        vec![
            14682783085930648838,
            7896655986299558210,
            9328642557612914244,
            6213125364180629684,
            16259136970573308007,
            12025260496935037210,
            1512031407150257270,
            1295709332547428576,
            13851880110872460625,
            6734559515296147531,
            17720805166223714561,
            11264121550751120724,
            7210341680607060660,
            17759718475616004694,
            610155440804635364,
            3209025413915748371,
        ],
    ]);

    // Each round has two round constants: `fst_rc` and `snd_rc`.
    // `fst_rc` values are indexed in the below array as:
    // `2 * round_number * register_counter + register_index`
    // `snd_rc` values are indexed in the below array as:
    // `2 * round_number * register_counter + register_index + 1`
    let round_constants_u128: Vec<u128> = vec![
        15139912583685767368,
        8372387753867525709,
        2183680717104184380,
        3244606960098905893,
        3147881977364597901,
        9452780775072264938,
        1082537058754139762,
        10970853375448447283,
        3062104324741241281,
        18009675040823690122,
        9709134112189744652,
        15857062738397365943,
        5016225506033072343,
        5216859395468346115,
        6462263614532073214,
        1493656999465165663,
        828768000476271392,
        262568612853428171,
        10113927829938360011,
        3228404466757125020,
        7320852123907649631,
        13627426656786462355,
        7964883404857794874,
        1407934150297697997,
        17336604982330804394,
        17906014506034551057,
        4632709206831589562,
        12999797106063314512,
        17162978498471467904,
        6827540927719713380,
        4753504633679017533,
        17716852809995758525,
        8549423660797843647,
        2362390356169006813,
        16716828864075537528,
        2740683348482332949,
        7756193835844677826,
        17543799665801483121,
        15002804793384601632,
        7902645524886711764,
        15165733099428544473,
        4077635361197762831,
        15132376188215154091,
        10741861618481937993,
        13707397012333257757,
        14226034480467186519,
        18245513484961172378,
        13273670281248631122,
        18251304196568320201,
        18190580491784411188,
        6118572220412064319,
        5630770511111509423,
        7970516069264861936,
        13449271048822160788,
        6851697376735269367,
        17987627018199535376,
        5294172762355915266,
        13844513406523115704,
        14597636171777994036,
        6061614115452481739,
        8186070796010445225,
        2327693164544063482,
        855935718254855095,
        10009207201287677622,
        10381177680823887718,
        18166133947715927863,
        17760506907335165396,
        3370764898316519938,
        5201580129905804035,
        1620223121525450629,
        14461318317868382163,
        1250929940922089768,
        13370612866774614255,
        7175470036866504098,
        16421684582717699126,
        16644320598987600726,
        17802798266780789487,
        6974241949143442442,
        17591712720223212489,
        16201733676622149735,
        286099893890784288,
        8057298197517276497,
        6444512502860040579,
        8347461167435943315,
        17352444114675313421,
        13535064425127211380,
        4772591666336791434,
        427985333074531866,
        14141791479819390154,
        7028809244427084468,
        9426904145082569174,
        6166111020063614179,
        8951223707117953234,
        3431064000345231130,
        1944155315841337325,
        6285600810995398496,
        16897111123465175857,
        4660909896474179791,
        18192626343736320364,
        5057838432340191471,
        14014302776583938723,
        9925254923879301551,
        6829435345780265556,
        8968794115294201104,
        17778545491689490446,
        18017797995365371861,
        18060766500386119579,
        12896732587303423715,
        4187616244444972880,
        10797712368247465599,
        5551515461716974377,
        5987237400880775150,
        8306936493309794552,
        10555482202024602033,
        16045656883318709119,
        14224667772707921698,
        7464515010550790466,
        14683637456755672385,
        8606694398702844028,
        12783325878688361611,
        10135605311909694521,
        6036681888442161456,
        13502595716772524386,
        17837288544072949135,
        16970790481274575856,
        12771951327386638665,
        7953144665513487435,
        10232601596097265370,
        7142562723872426447,
        7061326483481627814,
        2700322576799317485,
        6623246769381195291,
        16825539912038364772,
        17345255259493544461,
        3655344217194071236,
        4906781818047525714,
        14897453143374918047,
        12697105275305687091,
        6365510487307614865,
        16389921370395602280,
        6184292348425681997,
        1625734039805583227,
        7926303851971506844,
        6764450482313517598,
        12861725371095466098,
        1457318443242363431,
        6401144276852156944,
        11758577537140385015,
        7035279949079298611,
        17490109387633149109,
        9028549762556146425,
        14629064429955990677,
        7345978731773547933,
        2380447650891770049,
        13946626261179506153,
        14112757565552107369,
        18323048004349754740,
        3761840715850313303,
        2423761811055022202,
        4043073367058340954,
        4714747831891079272,
        9903324717460101691,
        16489681373737990564,
        12205142203164019145,
        7650721966187356479,
        13176636867741415622,
        8725940740195977648,
        7850051922002287223,
        7013216436240322065,
        7521500899942431357,
        17948709915499568560,
        12709968715340313663,
        12864870176028239567,
        13835492971050856940,
        14117813659377608536,
        17930389253653738705,
        16665999642411270287,
        8522764273244228281,
        17022385114623716012,
        17792533099449144220,
        9666141708192493561,
        4101243295111900354,
        11110149680511328320,
        15833373900081216881,
        2858902809543644288,
        15185937040593697757,
        1229742010402781808,
        12488915253710643809,
        14449445461821352645,
        11702325210632962260,
        7390229042372607295,
        13724660230648496560,
        16370078900053649525,
        6897898366117786971,
        12564585209779431146,
        15916465850680923114,
        3497319829092809455,
        3681935191724738445,
        17269401177087593182,
        14149218837807091766,
        13453529877505970461,
        15298165362714239682,
        14728462634044980354,
        14409721890326796259,
        17353894810846356075,
        16857127813837277773,
        11187357872695367332,
        15533140707195072093,
        1163405869960896591,
        15296392010875874377,
        17872716265685676772,
        14706935000063347212,
        14502717840925123585,
        1458466805797611569,
        2849079512899132391,
        14109081278228167673,
        8933669600131241369,
        8173386480957668450,
        15252826729106121549,
        10128993114764423519,
        11364771171604097376,
        14762095736262922188,
        13319725258546020263,
        16948750294723703018,
        10039494505766092885,
        14730563960989205668,
        16314543682302146762,
        13412588491336542421,
        5973689466852663000,
        673906515894578274,
        4039316712345686736,
        2031308080490921066,
        2907338798762025874,
        12316517814797934964,
        9307548410347506674,
        9351070955954520832,
        5794230072435402060,
        7922269617708021679,
        9708384153023840180,
        16472577099676318887,
        5244055413069805590,
        18123735486382626662,
        6519538476295982160,
        14228372996780660309,
        7960505044283116493,
        13993750470080027634,
        11478414004339098168,
        5009409638864158506,
        15807366605352652129,
        10685686439628572285,
        6800403862825412390,
        13138657193944784618,
        6448410590255081786,
        4381763274661386195,
        3646572817684127401,
        2916928929409428212,
    ];
    let round_constants: Vec<BFieldElement> = round_constants_u128
        .iter()
        .map(|elem| BFieldElement::new(*elem))
        .collect();

    RescuePrime {
        m: 16,
        steps_count: 7,
        alpha: 7,
        alpha_inv: 10540996611094048183,
        max_input_length: 10,
        output_length: 5,
        mds,
        mds_inv,
        round_constants,
    }
}

pub fn rescue_prime_medium_test_params() -> RescuePrime {
    let mds: Vec<Vec<BFieldElement>> = to_matrix(vec![
        vec![
            1473570182113651655,
            16634487879601443389,
            8045461079494518903,
            15409749493285347594,
            13775018642263716123,
            1945046876074400,
            18446743954022624421,
            960800,
        ],
        vec![
            176903157748902929,
            10792597897749413887,
            5984122101218315060,
            6527104246624281725,
            12513521381679785161,
            1008162094259635381,
            18337820521218738721,
            807744680100,
        ],
        vec![
            2550301768355404224,
            9715339608819428019,
            3680647216738219850,
            5612734856473481451,
            9034190253655852435,
            468907585716328870,
            5164209308626219394,
            667157540444234400,
        ],
        vec![
            14907362269659886741,
            12139441368366887768,
            5535072882885501002,
            11445856951218800405,
            4686477643633113661,
            3303150434670551547,
            17550214685576427196,
            4219400041647168965,
        ],
        vec![
            16451670062666211043,
            9602023591997199292,
            5651010992400911620,
            8176905273675888076,
            5702720516276885328,
            14893818261262814679,
            249245398227485579,
            13059582181150941668,
        ],
        vec![
            7833542642397320604,
            14689103784539147977,
            3438594086554781885,
            15268888213925842708,
            11939003815566714568,
            8989508280421368336,
            6156152656257464317,
            5472182797995696890,
        ],
        vec![
            7564263736652124188,
            8558130900587161902,
            736146750279383410,
            7016867986003173149,
            13429604656547943911,
            10651454574878384162,
            543217156874793024,
            6840546446420789218,
        ],
        vec![
            1649007993007911930,
            4152864938925844271,
            17219516348731002416,
            17818049165783807597,
            12021406670893231175,
            15720329315308004769,
            16528099385515401114,
            7124446528907718334,
        ],
    ]);
    let mds_inv: Vec<Vec<BFieldElement>> = to_matrix(vec![
        vec![
            736023694432405199,
            8182972243258059710,
            3498051006941954835,
            17162055534186847493,
            3768423433765138740,
            11503737910687402654,
            1453219729342325084,
            9035748655629619249,
        ],
        vec![
            15693091329388750858,
            2573505753597974022,
            15745159230526333895,
            6686448110042837653,
            4703662890575546906,
            9916617752772351520,
            5528098920813017414,
            12940392289941525017,
        ],
        vec![
            17745064910773280016,
            797434693060697420,
            5264502565591282528,
            13421417760019661442,
            2531125835183648429,
            10215801464183102177,
            13320883810044252580,
            10490745238802412693,
        ],
        vec![
            13493004860061390160,
            9141779445896724328,
            8663329072260147678,
            11949007539381617003,
            11905189282259646646,
            9053033456546638156,
            10555764414940492586,
            17472612275726265049,
        ],
        vec![
            17282312527066577647,
            3183627701138979025,
            17441045567048219569,
            14021888144787377217,
            68208443019285159,
            17083767579863181537,
            7523887749186067568,
            15628982634963233884,
        ],
        vec![
            9611158784006141084,
            1369172041706808649,
            15668078875424249322,
            12124654295076732612,
            11234713694116848515,
            15845246860400413992,
            6096425342602242220,
            1837526384324900891,
        ],
        vec![
            1704968375846022558,
            18421938878397970569,
            5460387332071102602,
            7170422060048660548,
            2601576556945010044,
            3102141129998913266,
            16413708698593952625,
            465089176342120752,
        ],
        vec![
            13178933673652877755,
            6530780806989649095,
            8973909305075134254,
            18308669672380093672,
            10779850691477795951,
            7037270482897111650,
            7258305005278376908,
            1719256639907298000,
        ],
    ]);
    let round_constants: Vec<BFieldElement> = vec![
        3644313421111751418,
        8538319628205282756,
        13882218533317409943,
        9721769469016685917,
        4201237500909935930,
        664106904409665489,
        3695595307233458142,
        12294133999635185978,
        1609166620809067151,
        1450003008964642807,
        17477646854028746239,
        10100774894557210711,
        11791815149834612692,
        2069715352953635745,
        12645764093060705721,
        5092764503033762362,
        82791346061954111,
        5339171792955937015,
        3266811259302088297,
        17247846061090889489,
        10088370949693242257,
        18078261313415050669,
        17251417841752911608,
        5961919346934624499,
        14220184609959099333,
        17663256223148757067,
        9653080093803516063,
        5926370752962933522,
        12043854427480775737,
        2750318521765498790,
        14690229100637708862,
        2088294515487799648,
        350144093069393294,
        16312259608290577922,
        9315988314644799737,
        1351751588824238851,
        7107106405508649006,
        2736549493398675976,
        3044121171749567928,
        10044497206451195482,
        10794091114516223636,
        17571778638079980817,
        12162751952050924637,
        3554159452411324036,
        8062774182542858650,
        14121486818854436109,
        7991067169132776987,
        8663155346792298615,
        3501179421803044183,
        9960025733511898860,
        16117084365974742988,
        12395385122858375585,
        10752770041909498401,
        2622571407504502492,
        13084271368920289311,
        18347657881122321660,
        3203156139239095379,
        17298228216362609190,
        14151819256702600489,
        17709726133297234870,
        16303832823577216503,
        4158850439430437571,
        9469614379695629410,
        141081152754150046,
        10385722110870356945,
        11402341336033786504,
        7730381366624107460,
        3292837992505946569,
        1034974741027288342,
        3516742269895054423,
        137255889365780124,
        9468697531597679972,
        3467326213092404727,
        17528915151509070350,
        3439602021365818991,
        8267562695154368118,
        18267937347189828341,
        11309898871426839633,
        12697504685781637364,
        14859949517310654046,
    ]
    .iter()
    .map(|c| BFieldElement::new(*c))
    .collect();

    RescuePrime {
        m: 8,
        steps_count: 5,
        alpha: 7,
        alpha_inv: 10540996611094048183,
        max_input_length: 1,
        output_length: 1,
        mds,
        mds_inv,
        round_constants,
    }
}

pub fn rescue_prime_small_test_params() -> RescuePrime {
    let mds: Vec<Vec<BFieldElement>> = to_matrix(vec![
        vec![18446744069414584314, 8],
        vec![18446744069414584265, 57],
    ]);
    let mds_inv: Vec<Vec<BFieldElement>> = to_matrix(vec![
        vec![8282211623002466431, 10164532446412117891],
        vec![2635249152773512047, 15811494916641072275],
    ]);
    let round_constants: Vec<BFieldElement> = vec![
        2332150738039522007,
        14509310212337904020,
        10912091569337447457,
        12063225951859934822,
        229787765437407343,
        2723460492895357694,
        8662091204479247631,
        1761576149899789034,
        18199069885388719343,
        5241040396775442401,
        10943084231530266127,
        2122639938091092999,
    ]
    .iter()
    .map(|c| BFieldElement::new(*c))
    .collect();

    RescuePrime {
        m: 2,
        steps_count: 3,
        alpha: 7,
        alpha_inv: 10540996611094048183,
        max_input_length: 1,
        output_length: 1,
        mds,
        mds_inv,
        round_constants,
    }
}

fn to_matrix(entries: Vec<Vec<u128>>) -> Vec<Vec<BFieldElement>> {
    entries
        .iter()
        .map(|row| row.iter().map(|elem| BFieldElement::new(*elem)).collect())
        .collect()
}
