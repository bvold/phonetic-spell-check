use crate::get_possible_corrections;
use rstest::*;
use std::time::Duration;

#[rstest]
#[case::laff("laff", "laugh")]
#[case::fone("fone", "phone")]
#[case::tonite("tonite", "tonight")]
#[case::tho("tho", "though")]
#[case::thru("thru", "through")]
#[case::ghoti("ghoti", "fish")]
#[case::kumpoot("kumpoot", "compute")]
#[case::kumpliant("kumpliant", "compliant")]
#[case::kompost("kompost", "compost")]
#[case::kumpos("compose", "compose")]
#[case::ubowt("ubowt", "about")]
#[case::kryteria("kryteria", "criteria")]
#[case::krismas("krismas", "christmas")]
// #[case::krismas("krismas", "Christmas")] // FIXME - should be able to handle capital letters as well
#[case::wun("won", "one")]
#[case::lite("lite", "light")]
#[case::acquire("akwire", "acquire")]
#[case::bizar("bizar", "bizarre")]
#[case::semetary("semetary", "cemetery")]
#[case::definitly("definitly", "definitely")]
#[case::ekstasi("ekstasi", "ecstasy")]
#[case::forin("forin", "foreign")]
#[case::gage("gage", "gauge")]
#[case::haras("haras", "harass")]
#[case::indipendunt("indipendunt", "independent")] //  FIXME test terminated by IDE
#[case::jewlry("jewlry", "jewelry")]
#[case::yon("yon", "yawn")]
#[case::epifany("epifany", "epiphany")]
// Schwa
#[case::abut("abut", "abut")]
#[case::silunt("silunt", "silent")]
#[case::marutim("marutim", "maritime")]
#[case::conekt("conekt", "connect")]
#[case::sirkus("sirkus", "circus")]
#[case::fizishun("fizishun", "physician")]
#[case::verandah("varandah", "veranda")]
#[case::kaptin("kaptin", "captain")]
#[case::oshun("oshun", "ocean")]
// #[case::mulun("mulun", "mullein")]  // not in CMU, not in /usr/share/dict/words
#[case::lunchun("lunchun", "luncheon")]
#[case::kulegit("kulegit", "collegiate")]
// #[case::waistcut("waistcut", "waistcoat")] // not in CMU, but is in /usr/share/dict/words
// #[case::funeshun("faneshun", "phoenician")]  // in CMU, not in /usr/share/dict/words
#[case::porpus("porpus", "porpoise")]
#[case::famus("famus", "famous")]
// #[case::pillakas("pillakas", "pillowcase")] // not in CMU
#[case::garila("garila", "gorilla")]
#[case::buricrat("burikrat", "bureaucrat")]
#[case::kazum("kazum", "chasm")]
#[case::mcoy("macoy", "mccoy")]
// AccentSchwa
#[case::wuz("wuz", "was")]
#[case::abuv("abuv", "above")]
#[case::humdrum("humdrum", "humdrum")]
// #[case::simric("simric", "cymric")] // not in CMU
#[case::duz("duz", "does")]
#[case::flud("flud", "flood")]
#[case::ruf("ruf", "rough")]
// SchwaR
#[case::lire("lire", "liar")] // FIXME get liyer to work
#[case::battr("battr", "batter")]
#[case::ilixr("ilixr", "elixir")]
#[case::oner("oner", "honor")]
#[case::ogr("ogr", "ogre")]
// #[case::injurr("injurr", "injurer")] // not in CMU, not in /usr/share/dict/words
#[case::martr("martr", "martyr")]
#[case::shofr("shofr", "chauffeur")]
#[case::glamr("glamr", "glamour")]
// AccessSchwaR
#[case::frn("frn", "fern")]
#[case::wir("wir", "were")]
#[case::berd("berd", "bird")]
#[case::werld("werld", "world")]
#[case::fer("fer", "fur")]
#[case::murtal("murtal", "myrtle")]
#[case::urth("urth", "earth")]
#[case::irr("irr", "err")]
#[case::skwerl("skwerl", "squirrel")]
#[case::jirnil("jirnil", "journal")]
#[case::huree("huree", "hurry")]
#[case::mir("mir", "myrrh")]
#[case::kurnl("kurnl", "colonel")]
// A
#[case::mat("mat", "mat")]
#[case::caf("caf", "calf")]
#[case::thar("thar", "there")]
// #[case::merang("marang", "meringue")]  // not in CMU, in /usr/share/dict/words
#[case::arial("arial", "aerial")]
#[case::plad("plad", "plaid")]
#[case::ar("ar", "air")]
#[case::art("ant", "aunt")]
#[case::prar("prar", "prayer")]
#[case::bar("bar", "bear")]
#[case::thar("thar", "their")]
// #[case::chat("chat", "chert")] // chat not in CMU, chert not in /usr/share/dict/words
// #[case::chert("churt", "chert")] // chert in CMU, chert not in /usr/share/dict/words
// LongA
#[case::fad("fad", "fade")]
// #[case::malay("malay", "melee")]  // in CMU, not in /usr/share/dict/words FIXME - add check to see if user wants to check /usr/share/dict/words or just trust CMU
#[case::mailstrum("mailstrum", "maelstrom")]
#[case::man("man", "main")]
#[case::strate("strate", "straight")]
#[case::strate("strate", "strait")]
// #[case::gail("gail", "gaol")]  // not in CMU, not in /usr/share/dict/words
#[case::gaje("gaje", "gauge")]
#[case::da("da", "day")]
#[case::stak("stak", "steak")]
// #[case::matinay("matinay", "matinee")]  // in CMU, not in /usr/share/dict/words
#[case::stak("stak", "steak")]
#[case::vain("vain", "vein")]
#[case::van("van", "vein")]
#[case::rane("rane", "reign")]
#[case::wa("wa", "weigh")]
#[case::pra("pra", "prey")]
// #[case::lonjeray("lonjeray", "lingerie")]  // CMU has a strange pronunciation, and could be incorrect, adding what I think it should be works
// #[case::atlander("atlander", "Uitlander")] // not in CMU, not in /usr/share/dict/words
// UmlautA
#[case::farther("farther", "farther")]
#[case::fathr("fathr", "father")]
#[case::gard("gard", "guard")]
// #[case::ontray("ontray", "entree")]  // in CMU, not in /usr/share/dict/words
#[case::sarjent("sarjent", "sergeant")]
#[case::cot("cot", "cot")]
#[case::cot("cot", "caught")]
#[case::bazar("bazar", "bazaar")]
#[case::sha("sha", "shah")]
#[case::notikl("notikl", "nautical")]
#[case::hart("hart", "heart")]
// #[case::not("not", "nought")] // not in CMU, in /usr/share/dict/words
#[case::burokrisi("burokrisi", "bureaucracy")] // FIXME takes too long for some permutations
#[case::patwa("patwa", "patois")]
// DotA
#[case::fother("fother", "father")]
// #[case::coff("coff", "calf")]  // CMU doesn't have this pronunciation
// #[case::cot("cot", "cart")] // CMU doesn't have this pronunciation
#[case::ont("ont", "aunt")]
// #[case::ont("baza", "bazaar")]  // CMU doesn't have this pronunciation (r-droppers)
// Au
#[case::sourkrowt("sourkrowt", "sauerkraut")]
#[case::lowd("lowd", "loud")]
#[case::bow("bow", "bough")]
#[case::nau("nau", "now")]
// different way to get there
// #[case::nau("kowchook", "caoutchouc")]  // not in CMU, not in /usr/share/dict/words
// ShortE
#[case::eny("eny", "any")]
#[case::enny("enny", "any")]
#[case::bet("bet", "bet")]
#[case::err("err", "err")]
#[case::er("er", "err")]
#[case::venilla("venilla", "vanilla")]
#[case::berry("berry", "bury")]
#[case::esthetic("esthetic", "aesthetic")]
#[case::erial("erial", "aerial")]
#[case::erial("sed", "said")]
#[case::er("er", "air")]
#[case::sez("sez", "says")]
#[case::prer("prer", "prayer")]
#[case::ber("ber", "bear")]
#[case::bred("bred", "bread")]
#[case::ther("ther", "their")]
#[case::heffer("heffer", "heifer")]
#[case::lepperd("lepperd", "leopard")]
#[case::frend("frend", "friend")]
// #[case::fetid("fetid", "foetid")]  // Not in CMU, not in /usr/share/dict/words
// #[case::letenant("letenant", "lieutenant")]  // This pronunciation not in CMU
#[case::frend("frend", "friend")]
// #[case::embira("embira", "mbira")]  // Incorrect pronunciation in CMU

// LongE
#[case::mee("mee", "me")]
#[case::skee("skee", "ski")]
#[case::prittee("prittee", "pretty")]
// #[case::eon("eon", "aeon")]  // Not in CMU, is in /usr/share/dict/words
#[case::mundee("mundee", "monday")]
#[case::eezee("eezee", "easy")]
#[case::beet("beet", "beet")]
#[case::see("see", "see")]
#[case::reseev("reseev", "receive")]
#[case::peeple("peeple", "people")]
#[case::kee("kee", "key")]
#[case::greef("greef", "grief")]
#[case::feebee("feebee", "phoebe")]
// #[case::feetid("feetid", "foetid")]  // Not in CMU, not in /usr/share/dict/words
// #[case::shelalee("shelalee", "shillelagh")]  // Not in CMU, in /usr/share/dict/words, FIXME - test hangs
// #[case::shelalee("shelalee", "shillalah")]  // Not in CMU, in /usr/share/dict/words, FIXME - test hangs
// #[case::shamee("shamee", "chamois")]   // Not in CMU, in /usr/share/dict/words
// #[case::shamwa("shamwa", "chamois")]  // Not in CMU, in /usr/share/dict/words
// ShortI
#[case::homij("homij", "homage")]
// #[case::katikorner("katikorner", "catercorner")] // Not in CMU, in /usr/share/dict/words
#[case::ingland("ingland", "england")]
// #[case::ingland("ingland", "England")]  // FIXME - handle capital letters correctly
#[case::pritty("pritty", "pretty")]
#[case::sirius("sirius", "serious")] // FIXME - maybe a reverse parse would work well for this many vowels
#[case::tip("tip", "tip")]
#[case::wimin("wimin", "women")]
#[case::bizee("bizee", "busy")]
#[case::mith("mith", "myth")]
#[case::hir("hir", "hear")]
#[case::bin("bin", "been")]
#[case::bir("bir", "beer")]
#[case::crick("crick", "creek")]
#[case::counterfit("counterfit", "counterfeit")] // FIXME - test hangs
#[case::wird("wird", "weird")]
// #[case::carrij("carrij", "carriage")]  // FIXME
#[case::carij("carij", "carriage")]
#[case::siv("siv", "sieve")]
#[case::bilding("bilding", "building")]
// LongI
#[case::si("si", "sigh")]
#[case::gid("gid", "guide")]
#[case::sli("sli", "sly")]
#[case::ile("ile", "aisle")]
// #[case::ioli("ioli", "aioli")] // Not in CMU, not in /usr/share/dict/words
#[case::biyu("biu", "bayou")]
// #[case::biyu("biyu", "bayou")]  // FIXME
#[case::papia("papia", "papaya")]
// #[case::papiya("papiya", "papaya")]  // FIXME
#[case::hist("hist", "heist")]
#[case::hite("hite", "height")]
#[case::giser("giser", "geyser")]
#[case::li("li", "lie")]
#[case::kiote("kiote", "coyote")]
#[case::bi("bi", "buy")]
#[case::di("di", "dye")]
#[case::i("i", "aye")]
#[case::i("i", "eye")]
#[case::kwire("kwire", "choir")]
#[case::kwier("kwier", "choir")]
// LongO
#[case::kwahog("kwahog", "quahog")]
#[case::kupolo("kupolo", "cupola")]
#[case::bon("bon", "bone")]
#[case::fok("fok", "folk")]
#[case::farow("farow", "pharaoh")]
#[case::pharaoh("pharaoh", "pharaoh")]
#[case::shovinist("shovinist", "chauvinist")]
#[case::showvinist("showvinist", "chauvinist")] // FIXME - test hangs
#[case::yomin("yomin", "yeoman")]
#[case::so("so", "sew")]
#[case::cot("cot", "coat")]
#[case::x_do("do", "doe")]
#[case::o("o", "oh")]
#[case::no("no", "noh")]
#[case::broch("broch", "brooch")]
#[case::bolder("bolder", "boulder")]
#[case::tho("tho", "though")]
#[case::no("no", "know")]
// #[case::kwier("kwohog", "quahog")]  // Pronunciation not in CMU
#[case::platow("platow", "plateau")]
// #[case::buro("buro", "burgh")]  // Pronunciation not in CMU, Scottish form
#[case::buro("buro", "borough")]
// DotO
#[case::bol("bol", "ball")]
#[case::tok("tok", "talk")]
#[case::soft("soft", "soft")]
#[case::uto("uto", "utah")]
#[case::sosage("sosage", "sausage")]
#[case::cot("cot", "caught")]
#[case::so("so", "saw")]
#[case::gorjin("gorjin", "georgian")]
#[case::brod("brod", "broad")]
#[case::coff("coff", "cough")]
#[case::thot("thot", "thought")]
#[case::toward("toward", "toward")]
// Oi
#[case::loir("loir", "lawyer")]
// #[case::froidian("froidian", "freudian")]  // Incorrect pronunciation in CMU
#[case::coin("coin", "coin")]
#[case::boi("boi", "boy")]
// #[case::boi("boi", "buoy")]  // CPU doesn't have this pronunciation, it has "bui"
#[case::soing("soing", "sawing")]
// UmlautU
#[case::du("du", "do")]
#[case::muv("muv", "move")]
#[case::tu("tu", "two")]
#[case::flu("flu", "flu")]
// #[case::cruth("cruth", "crwth")]  // Not in CMU, not in /usr/share/dict/words
// #[case::sum("sum", "cwm")]  // Not in CMU, not in /usr/share/dict/words
#[case::rumatism("rumatizm", "rheumatism")]
#[case::manuver("manuver", "maneuver")]
#[case::cru("cru", "crew")]
#[case::shu("shu", "shoe")]
#[case::skul("skul", "school")]
#[case::uth("uth", "youth")]
#[case::thru("thru", "through")]
#[case::blu("blu", "blue")]
#[case::cruz("cruz", "cruise")]
// #[case::luword("luword", "leeward")]  // Pronunciation not in CMU
#[case::lutenent("lutenent", "lieutenant")]
// #[case::manuver("manuver", "manoeuvre")]  // Not in CMU, not in /usr/share/dict/words
#[case::buty("buty", "beauty")]
#[case::yu("yu", "ewe")]
// #[case::pyuit("pyuit", "peewit")]   // Not in CMU, not in /usr/share/dict/words
// DotU
#[case::wuman("wuman", "woman")]
#[case::wulf("wulf", "wolf")]
#[case::pull("pull", "pull")]
#[case::wud("wud", "wood")]
#[case::cud("cud", "could")]
// B
#[case::baby("baby", "baby")]
#[case::goberment("goberment", "government")]
#[case::seben("seben", "seven")]
#[case::ruber("ruber", "rubber")]
// #[case::bang("bang", "bhang")]  // Not in CMU, no in /usr/share/dict/words
#[case::cubbard("cubbard", "cupboard")]
#[case::razberry("razberry", "raspberry")]
// Ch
#[case::chello("chello", "cello")]
#[case::chin("chin", "chin")]
#[case::chek("chek", "czech")]
#[case::tenchun("tenchun", "tension")]
#[case::richous("richous", "righteous")]
#[case::queschun("queschun", "question")]
#[case::mach("mach", "match")]
#[case::nacher("nacher", "nature")]
// D
#[case::did("did", "did")]
#[case::ladder("ladder", "ladder")]
// #[case::dow("dow", "dhow")]  // not in CMU, not in /usr/share/dict/words
#[case::seemd("seemd", "seemed")]
#[case::wood("wood", "would")]
// F
#[case::fan("fan", "fan")]
#[case::ofer("ofer", "offer")]
#[case::laf("laf", "laugh")]
#[case::caf("caf", "calf")]
#[case::telefon("telefon", "telephone")]
// G
#[case::go("go", "go")]
#[case::eg("eg", "egg")]
#[case::gost("gost", "ghost")]
#[case::gid("gid", "guide")]
#[case::plag("plag", "plague")]
#[case::igzampl("igzampl", "example")]
// H
// #[case::hila("hila", "gila")]  // CMU doesn't have this pronunciation
#[case::gost("hat", "hat")]
// #[case::hai("hai", "jai")]  // Not in /usr/share/dict/words
#[case::hanuka("hanuka", "chanukah")]
#[case::hoo("hoo", "who")]
// Hw
#[case::hwon("hwon", "juan")]
#[case::mariwana("mariwana", "marijuana")] // FIXME test terminated
#[case::hwale("hwale", "whale")]
#[case::hwen("hwhen", "when")]
// J
#[case::jem("jem", "gem")]
#[case::joy("joy", "joy")]
// #[case::grenij("grenij", "greenwich")]  // CMU doesn't have this pronunciation  FIXME add additional pronunciations that CMU doesn't have
#[case::bujit("bujit", "budget")]
#[case::brij("brij", "bridge")]
#[case::soljer("soljer", "soldier")]
#[case::adjective("adjective", "adjective")]
// FIXME test terminated
// #[case::exajerate("exajerate", "exaggerate")] // FIXME Test doesn't produce any possibilities
#[case::rejun("rejun", "region")]
// #[case::haj("haj", "hajj")]  // In CMU, not in /usr/share/dict/words
#[case::grajuation("grajuation", "graduation")]
// K
#[case::katch("katch", "catch")]
#[case::kid("kid", "kid")]
#[case::take("take", "take")]
#[case::kwit("kwit", "quit")]
#[case::akount("akount", "account")]
#[case::kaos("kaos", "chaos")]
// #[case::lok("lok", "loch")]  // In CMU, not in /usr/share/dict/words
#[case::skism("skism", "schism")]
#[case::pik("pik", "pick")]
#[case::akwire("akwire", "acquire")]
#[case::biskit("biskit", "biscuit")]
// #[case::lok("lok", "lough")]  // Wrong pronunciation in CMU, not in /usr/share/dict/words
#[case::kaki("kaki", "khaki")]
// #[case::chuka("chuka", "chukka")]  // Not in CMU, not in /usr/share/dict/words
// #[case::pucka("pucka", "pukka")]  // Not in CMU, not in /usr/share/dict/words
#[case::tok("tok", "talk")]
#[case::likker("likker", "liquor")]
#[case::plak("plak", "plaque")]
#[case::kay("kay", "quay")]
#[case::sakkerine("sakkerine", "saccharine")] // FIXME IDE terminates
#[case::lakker("lakker", "lacquer")]
#[case::taks("taks", "tax")]
// UnderlineK  // These are mostly foreign words, probably not in CMU
// #[case::khanukka("khanukka", "hanukkah")]
// #[case::khasid("khasid", "hasid")]
// #[case::lokh("lokh", "loch")]
// #[case::khasid("khasid", "chasid")]
// #[case::lokh("lokh", "lough")]  // This pronunciation not in CMU, not in /usr/share/dict/words
// L
#[case::lo("low", "low")]
#[case::sal("sal", "sale")]
#[case::filing("filing", "filling")]
// #[case::fail("fail", "faille")]   // Not in CMU, not in /usr/share/dict/words
// #[case::laza("laza", "lhasa")]   // Not in CMU, in /usr/share/dict/words
// #[case::kil("kil", "kiln")]   // This pronunciation not in CMU
// SchwaL
// #[case::derndl("derndl", "dirndl")]  // #[case::fail("fail", "faille")]   // Not in CMU, not in /usr/share/dict/words
#[case::pedl("pedl", "pedal")]
// #[case::betl("betl", "betel")]  // In CMU, not in /usr/share/dict/words
#[case::battl("battl", "battle")]
// #[case::fenl("fenl", "phenyl")]  // Not in CMU, not in /usr/share/dict/words
// M
#[case::me("me", "me")]
#[case::kum("kum", "come")]
#[case::flem("flem", "phlegm")]
#[case::com("com", "calm")]
#[case::kom("kom", "comb")]
// #[case::mo("mo", "mho")]  // Not in CMU, not in /usr/share/dict/words
#[case::dumy("dumy", "dummy")]
#[case::otum("otum", "autumn")]
// #[case::drom("drom", "drachm")]  // Not in CMU, not in /usr/share/dict/words
// SchwaM
// #[case::opum("opum", "open")]  // This pronunciation not in CMU
// #[case::hapum("hapum", "happen")] // This pronunciation not in CMU
// #[case::captum("captum", "captain")]  // This pronunciation not in CMU
// #[case::govument("govument", "government")]  // This pronunciation not in CMU
// N
#[case::no("no", "no")]
#[case::alon("alon", "alone")]
#[case::sine("sine", "sign")]
#[case::nat("nat", "gnat")]
#[case::rane("rane", "reign")]
#[case::not("not", "knot")]
#[case::nemonic("nemonic", "mnemonic")]
// #[case::comtroller("comtroller", "comptroller")]  // FIXME Test terminated by IDE
#[case::banner("banner", "banner")]
#[case::numonia("numonia", "pneumonia")]
// SchwaN
#[case::sudun("sudun", "sudden")]
#[case::kotun("kotun", "cotton")]
#[case::certun("certun", "certain")]
// Ng
#[case::ingk("ingk", "ink")]
#[case::finger("finger", "finger")]
// #[case::orangutang("orangutang", "orangutan")]  // CMU doesn't have this pronunciation, which appears to be American English
#[case::hangkerchief("hangkerchief", "handkerchief")] // FIXME IDE terminates test
#[case::sing("sing", "sing")]
// #[case::mahjong("mahjong", "mahjongg")]  // Not in CMU, /usr/share/dict/words has mahjong
#[case::harang("harang", "harangue")]
// P
#[case::port("port", "port")]
#[case::stop("stop", "stop")]
#[case::ap("ape", "ape")]
#[case::sheperd("sheperd", "shepherd")]
// #[case::dipthong("dipthong", "diphthong")]  // CMU doesn't have this pronunciation, but in Collins, M-W, Cambridge, Oxford, Am. Her
#[case::x_super("super", "supper")]
// R
#[case::red("red", "red")]
#[case::care("care", "care")]
#[case::card("card", "card")]
#[case::sheperd("sheperd", "shepherd")]
#[case::rime("rime", "rhyme")]
#[case::mary("mary", "merry")]
#[case::rite("rite", "write")]
#[case::diaria("diaria", "diarrhea")]
#[case::kernel("kernel", "colonel")]
// S
#[case::prosede("prosede", "proceed")]
#[case::rase("rase", "race")]
#[case::say("say", "say")]
#[case::luse("luse", "loose")]
#[case::pretsel("pretsel", "pretzel")]
#[case::salm("salm", "psalm")]
#[case::fasinate("fasinate", "fascinate")]
#[case::sizors("sizors", "scissors")]
#[case::mas("mas", "mass")]
#[case::lisin("lisin", "listen")]
#[case::chrismas("chrismas", "christmas")]
#[case::sar("sar", "tsar")]
// #[case::sar("sar", "tzar")]   // CMU doesn't have this pronunciation
// #[case::sism("sism", "schism")]    // CMU doesn't have this pronunciation
#[case::ismus("ismus", "isthmus")]
#[case::taks("taks", "tax")]
// Sh
#[case::oshianic("oshianic", "oceanic")] // FIXME IDE terminates test
#[case::shugar("shugar", "sugar")]
#[case::shure("shure", "sure")]
#[case::mashine("mashine", "machine")]
#[case::speshial("speshal", "special")]
#[case::fashism("fashism", "fascism")]
#[case::nashus("nashus", "nauseous")] // FIXME IDE terminates test
#[case::shy("shy", "shy")]
#[case::emulshun("emulshun", "emulsion")]
#[case::ski("ski", "ski")]
#[case::tishu("tishu", "tissue")]
#[case::nashun("nashun", "nation")]
// #[case::marshioness("marshioness", "marchioness")]     // CMU doesn't have this pronunciation
// #[case::shaw("shaw", "pshaw")]    // CMU doesn't have this pronunciation
#[case::shist("shist", "schist")]
#[case::conshus("conshus", "conscious")]
#[case::mishun("mishun", "mission")]
// #[case::fusha("fusha", "fuchsia")]   // Not in CMU, in /usr/share/dict/words
// T
#[case::tea("tee", "tea")]
#[case::eet("eet", "eat")]
#[case::lat("lat", "late")]
#[case::det("det", "debt")]
// #[case::stenoid("stenoid", "ctenoid")]     // Not in CMU, in /usr/share/dict/words
#[case::wokt("wokt", "walked")]
#[case::tomain("tomain", "ptomaine")]
#[case::riseet("riseet", "receipt")]
// #[case::tim("tim", "thyme")]   // This pronunciation is not in CMU, but this is actually the preferred pronunciation
#[case::tomas("tomas", "thomas")]
#[case::tie("tie", "thai")]
#[case::buton("buton", "button")]
#[case::yot("yot", "yacht")]
#[case::nit("nit", "night")]
#[case::strate("strate", "straight")]
// #[case::tizik("tizik", "phthisic")]   // Not in CMU, not in /usr/share/dict/words
// Th
// #[case::troth("troth", "trough")]  // This pronunciation is not in CMU
#[case::tie("thin", "thin")]
#[case::breath("breath", "breath")]
#[case::drowth("drowth", "drought")]
// #[case::thonic("thonic", "chthonic")]   // Not in CMU, not in /usr/share/dict/words
// #[case::thaline("thaline", "phthalein")]  // Not in CMU, not in /usr/share/dict/words
// UnderlineTh
// #[case::istethfod("istethfod", "eisteddfod")]  // Not in CMU, not in /usr/share/dict/words
// #[case::eth("eth", "edh")]  // Not in CMU, not in /usr/share/dict/words
#[case::this("this", "this")]
#[case::teething("teething", "teething")]
#[case::breath("breath", "breathe")]
// V
#[case::uv("uv", "of")]
#[case::vary("vary", "very")]
#[case::sav("sav", "save")]
// #[case::vadeln("vadeln", "wedeln")]  // Not in CMU, not in /usr/share/dict/words
#[case::steven("steven", "stephen")]
#[case::savy("savy", "savvy")]
// W
#[case::perswade("perswade", "persuade")]
#[case::kwit("kwit", "quit")]
#[case::wa("wa", "way")]
#[case::won("won", "juan")]
#[case::mariwona("mariwona", "marijuana")]
// #[case::wabain("wabain", "ouabain")]   // Not in CMU, not in /usr/share/dict/words
#[case::bivwak("bivwak", "bivouac")]
#[case::wale("wale", "whale")]
#[case::wen("wen", "when")]
#[case::wun("wun", "one")]
#[case::kwir("kwir", "choir")]
#[case::patwa("patwa", "patois")]
#[case::strenwus("strenwus", "strenuous")]
// Y
#[case::opinyun("opinyun", "opinion")]
#[case::haleluya("haleluya", "hallelujah")] // IDE terminates test
#[case::yard("yard", "yard")]
#[case::byuty("byuty", "beauty")]
// #[case::canyon("canyon", "cañon")]  // Not in CMU, not in /usr/share/dict/words because it's Spanish
#[case::cyut("cyut", "cute")]
#[case::fyud("fyud", "feud")]
#[case::fyu("fyu", "few")]
// #[case::strenyawus("strenyawus", "strenuous")]  // This pronunciation is not in CMU
// #[case::tortiya("tortiya", "tortilla")] // This pronunciation is not in CMU, /tɔɹˈtiə/
#[case::yunit("yunit", "unit")]
// Z
#[case::daz("daz", "days")]
#[case::wuz("wuz", "was")]
#[case::pleez("pleez", "please")]
#[case::zylofon("zylofon", "xylophone")]
// #[case::batoz("batoz", "bateaux")]  // Not in CMU, not in /usr/share/dict/words
#[case::zon("zon", "zone")]
#[case::zar("haz", "haze")]
#[case::zon("zar", "czar")]
#[case::dizern("dizern", "discern")]
#[case::sizors("sizors", "scissors")]
#[case::zar("zar", "tsar")]
// #[case::zar("zar", "tzar")]  // This pronunciation is not in CMU
#[case::buz("buz", "buzz")]
#[case::igzampl("igzampl", "example")]
// Zh
// #[case::fizin("fizin", "fission")]  // This pronunciation is not in CMU, /ˈfɪʃən/
#[case::azer("azer", "azure")]
#[case::mezer("mezer", "measure")]
// Silent
#[case::com("com", "comb")] // b
#[case::det("det", "debt")] // b
#[case::konetikut("konetikut", "connecticut")] // c
// #[case::stenoid("stenoid", "ctenoid")]     // Not in CMU, in /usr/share/dict/words
#[case::yot("yot", "yacht")] // ch
// #[case::thonic("thonic", "chthonic")] // ch   // Not in CMU, not in /usr/share/dict/words
#[case::dat("dat", "date")] // e
#[case::liv("liv", "live")] // e
#[case::battl("battl", "battle")] // e
#[case::seemd("seemd", "seemed")] // e
#[case::plag("plag", "plague")] // e
#[case::nat("nat", "gnat")] // g
#[case::sin("sin", "sign")] // g
#[case::diafram("diafram", "diaphragm")] // g
#[case::nit("nit", "night")] // gh
#[case::strat("strat", "straight")] // gh
#[case::tho("tho", "though")] // gh
#[case::thot("thot", "thought")] // gh
#[case::ower("ower", "hour")] // h
#[case::onor("onor", "honor")] // h
#[case::rim("rim", "rhyme")]
// h
// #[case::tim("tim", "thyme")] // h  // This pronunciation is not in CMU, but this is actually the preferred pronunciation
#[case::biznis("biznis", "business")] // i
#[case::parlament("parlament", "parliament")] // i
#[case::not("not", "knot")] // k
#[case::no("no", "know")] // k
#[case::tok("tok", "talk")] // l
#[case::fok("fok", "folk")] // l
#[case::wood("wood", "would")] // l
#[case::com("com", "calm")] // l
#[case::nemonic("nemonic", "mnemonic")] // m
#[case::otum("otum", "autumn")] // n
#[case::goverment("goverment", "government")] // n
// #[case::kil("kil", "kiln")]  // n   // This pronunciation is not in CMU
#[case::sofmor("sofmor", "sophomore")]
// o
// #[case::posum("posum", "opossum")]  // l  // Treated as separate words in CMU and /usr/share/dict/words
#[case::cubard("cubard", "cupboard")] // p
#[case::numonia("numonia", "pneumonia")] // p
#[case::salm("salm", "psalm")] // p
#[case::tomain("tomain", "ptomaine")] // p
#[case::razbery("razbery", "raspberry")]
// p
// #[case::thaline("thaline", "phthalein")] // ph // Not in CMU, not in /usr/share/dict/words
// #[case::tizik("tizik", "phthisic")]   // ph  // Not in CMU, not in /usr/share/dict/words
#[case::supriz("supriz", "surprise")] // r
#[case::il("il", "aisle")] // s
#[case::iland("iland", "island")] // s
#[case::patwa("patwa", "patois")] // s
#[case::daman("daman", "demesne")]
// s
// #[case::bera("bera", "beret")]  // t   // CMU has incorrect pronunciation
// #[case::bosun("bosun", "boatswain")]  // t  // CMU has incorrect pronunciation
#[case::chrismas("chrismas", "christmas")] // t
#[case::depo("depo", "depot")] // t
#[case::lisen("lisen", "listen")] // t
#[case::azma("azma", "asthma")] // th
#[case::izmus("izmus", "isthmus")]
// th
// #[case::norester("norester", "northeaster")]  // th  // not in CMU, is in /usr/share/dict/words
#[case::bizkit("bizkit", "biscuit")] // l
#[case::bild("bild", "build")] // u
#[case::gest("gest", "guest")]
// l
// #[case::pursivant("pursivant", "pursuivant")]  // u  // not in CMU, not in /usr/share/dict/words
#[case::qay("qay", "quay")] // u
#[case::plag("plag", "plague")] // u
#[case::plaq("plaq", "plaque")] // u
#[case::hu("hu", "who")] // l
#[case::rit("rit", "write")] // l
#[case::to("to", "two")] // w
#[case::sord("sord", "sword")]
// w
// #[case::bosun("bosun", "boatswain")]  // w  // In CMU with incorrect pronunciation
// #[case::fo("fo", "faux")]  // x   // In CMU with incorrect pronunciation, not in /usr/share/dict/words
// #[case::batoz("batoz", "bateaux")]  // Not in CMU, not in /usr/share/dict/words
// #[case::est("est", "yeast")]  // y   // CMU doesn't have this pronuncication and it's uncommon - and none of the dicts have this pronunciation
#[case::rondavu("rondavu", "rendezvous")] // z
pub fn test_phonetic_spell_check(#[case] original_word: &str, #[case] expected_correction: &str) {
    let possible_corrections = get_possible_corrections(original_word.to_string());
    assert!(possible_corrections.contains(expected_correction));
}
