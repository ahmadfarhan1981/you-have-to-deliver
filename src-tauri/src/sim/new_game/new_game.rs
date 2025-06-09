use crate::sim::person::spawner::TalentGrade;
use crate::sim::person::spawner::TalentGrade::{Apt, Basic, Brilliant, Exceptional, Gifted, Sharp};
use std::string::ToString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewGameConfig {
    company_name: String,
    company_slogan: String,
    initial_starting_employee_config: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StartingEmployeesConfig {
    pub name: String,
    pub config: Vec<(TalentGrade, u16)>,
}
impl Default for StartingEmployeesConfig {
    fn default() -> Self {
        Self {
            name: "Standard company".to_string(),
            config: vec![
                (Basic, 6),
                (Apt, 8),
                (Sharp, 3),
                (Gifted, 2),
                (Brilliant, 1),
                (Exceptional, 0),
            ],
        }
    }
}

#[tauri::command]
pub fn get_starting_employee_configs() -> Vec<StartingEmployeesConfig> {
    vec![

        StartingEmployeesConfig {
            name: "Standard company".to_string(),
            config: vec![
                (Basic, 6),
                (Apt, 8),
                (Sharp, 3),
                (Gifted, 2),
                (Brilliant, 1),
                (Exceptional, 0),
            ],
        },
        StartingEmployeesConfig {
            name: "We got a bunch of interns".to_string(),
            config: vec![(Apt, 5), (Basic, 25)],
        },
        StartingEmployeesConfig {
            name: "Small gifted team".to_string(),
            config: vec![(Gifted, 4)],
        },
        StartingEmployeesConfig {
            name: "One man show".to_string(),
            config: vec![(Exceptional, 1)],
        },
    ]
}

#[tauri::command]
pub fn get_company_presets() -> Vec<CompanyPresetStatic> {
    COMPANY_PRESETS.to_vec()
}


#[derive(Clone, Serialize, Deserialize)]
pub struct CompanyPresetStatic {
    pub name: &'static str,
    pub slogan: &'static str,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompanyPreset {
    pub name:String,
    pub slogan: String,
}
impl Default for CompanyPresetStatic {
    fn default() -> Self {
        Self{ name: "Paan's Dev House", slogan: "Making unnecessarily complex text games." }
    }
}
impl Default for CompanyPreset {
    fn default() -> Self {
        Self{ name: "Paan's Dev House".to_string(), slogan: "Making unnecessarily complex text games.".to_string() }
    }
}


pub const COMPANY_PRESETS: &[CompanyPresetStatic] = &[
    CompanyPresetStatic { name: "Paan's Dev House", slogan: "Making unnecessarily complex text games." },
    CompanyPresetStatic { name: "Bit Quarry", slogan: "Mining ideas. Building empires." },
    CompanyPresetStatic { name: "StackForge", slogan: "Code hard. Ship fast." },
    CompanyPresetStatic { name: "Pixel & Co.", slogan: "Design the difference." },
    CompanyPresetStatic { name: "Quantum Crate", slogan: "Think outside the tick." },
    CompanyPresetStatic { name: "Null Theory Labs", slogan: "Nothing ventured, everything gained." },
    CompanyPresetStatic { name: "Echo Sigma", slogan: "Make noise. Leave a mark." },
    CompanyPresetStatic { name: "Blue Crab Digital", slogan: "Soft shells. Hard code." },
    CompanyPresetStatic { name: "Overclocked Hearts", slogan: "Powered by passion, cooled by ambition." },
    CompanyPresetStatic { name: "ByteSmith Union", slogan: "Hammering out beautiful software." },
    CompanyPresetStatic { name: "Crimson Beta", slogan: "Bleeding-edge by design." },
    CompanyPresetStatic { name: "Lazy Lambda", slogan: "Because recursion is beautiful." },
    CompanyPresetStatic { name: "Syntax & Synapse", slogan: "Where ideas compile." },
    CompanyPresetStatic { name: "DryRun Interactive", slogan: "Test. Fail. Build." },
    CompanyPresetStatic { name: "Ternary Systems", slogan: "Yes. No. Maybe. We handle it." },
    CompanyPresetStatic { name: "Coffee++", slogan: "Caffeinate. Iterate. Dominate." },
    CompanyPresetStatic { name: "Fuzzy Logic Inc.", slogan: "It's not a bug, it's nuance." },
    CompanyPresetStatic { name: "Rust and Stardust", slogan: "Safe, fast, beautiful code." },
    CompanyPresetStatic { name: "Paperclip Initiative", slogan: "Automate everything. Especially ambition." },
    CompanyPresetStatic { name: "Oneiric Stack", slogan: "Build the dream. Debug the nightmare." },
    CompanyPresetStatic { name: "Bare Metal Hearts", slogan: "Closer to the machine, closer to truth." },
    CompanyPresetStatic { name: "Moonshot Gravy", slogan: "Big ideas. Smooth delivery." },
    CompanyPresetStatic { name: "Weird Normal LLC", slogan: "Business as unusual." },
    CompanyPresetStatic { name: "Fathom", slogan: "Depth in every decision." },
    CompanyPresetStatic { name: "Devils in the Details", slogan: "We read the specs. Twice." },
    CompanyPresetStatic { name: "Obsidian Workflow", slogan: "Unbreakable routines. Beautiful results." },
    CompanyPresetStatic { name: "WeylandYutani Corp", slogan: "Building better people." },
    CompanyPresetStatic { name: "OmniSyn", slogan: "Everything. Integrated." },
    CompanyPresetStatic { name: "404 Founders", slogan: "Startup not found." },
    CompanyPresetStatic { name: "The Merge Conflict Co.", slogan: "Our branches are better together." },
    CompanyPresetStatic { name: "NullPointer Solutions", slogan: "We do nothing, flawlessly." },
    CompanyPresetStatic { name: "Infinite Loop Labs", slogan: "We’ll get there. Eventually." },
    CompanyPresetStatic { name: "Foobar Group", slogan: "Standing in for greatness." },
    CompanyPresetStatic { name: "Stack Underrun", slogan: "Dangerously efficient." },
    CompanyPresetStatic { name: "GigaChad Systems", slogan: "Built different." },
    CompanyPresetStatic { name: "PipeDream Softworks", slogan: "It *might* work." },
    CompanyPresetStatic { name: "Promptly Wrong Inc.", slogan: "Confident answers, questionable accuracy." },
    CompanyPresetStatic { name: "ZebraStripes Technologies", slogan: "No two features alike." },
    CompanyPresetStatic { name: "YAGNI Works", slogan: "You ain’t gonna need it. Until you do." },
    CompanyPresetStatic { name: "Scrum & Burn", slogan: "Agile. Just not *too* agile." },
    CompanyPresetStatic { name: "Big O Bros.", slogan: "We optimize later." },
    CompanyPresetStatic { name: "Cargo Cult Creatives", slogan: "It worked in prod. Once." },
    CompanyPresetStatic { name: "Mainframe Revival", slogan: "Retro tech. Real respect." },
    CompanyPresetStatic { name: "Rubber Duck Ventures", slogan: "Explain it. Ship it." },
    CompanyPresetStatic { name: "Quantum Sandwich", slogan: "Both lunch and not lunch." },
    CompanyPresetStatic { name: "Second System Studio", slogan: "Over-engineering done right." },
    CompanyPresetStatic { name: "Grayscale & Co.", slogan: "Not everything has to pop." },
    CompanyPresetStatic { name: "The Off-By-One Group", slogan: "Almost always right." },
    CompanyPresetStatic { name: "Devs of the Round Table", slogan: "All commits are equal." },
    CompanyPresetStatic { name: "Ember & Ash", slogan: "Rise. Refactor. Repeat." },
    CompanyPresetStatic { name: "Updog Solutions", slogan: "What’s up with your workflow?" },
    CompanyPresetStatic { name: "N+1 Works", slogan: "Queries? We love them." },
    CompanyPresetStatic { name: "Decompile Me Softly", slogan: "Reverse engineering with care." },
    CompanyPresetStatic { name: "Postmortem Systems", slogan: "It was working yesterday." },
    CompanyPresetStatic { name: "Aether Dynamics", slogan: "Innovating the unseen." },
    CompanyPresetStatic { name: "Synapse Collective", slogan: "Connecting minds, crafting futures." },
    CompanyPresetStatic { name: "Apex Ascent", slogan: "Reaching new heights in tech." },
    CompanyPresetStatic { name: "Chronos Labs", slogan: "Mastering the flow of time and data." },
    CompanyPresetStatic { name: "Veridian Tech", slogan: "Growing green solutions with cutting-edge code." },
    CompanyPresetStatic { name: "Epoch Systems", slogan: "Defining the next era of development." },
    CompanyPresetStatic { name: "GigaGrid Solutions", slogan: "Building the backbone of tomorrow." },
    CompanyPresetStatic { name: "Lumen Works", slogan: "Shedding light on complex problems." },
    CompanyPresetStatic { name: "Nebula Nexus", slogan: "Connecting the universe of data." },
    CompanyPresetStatic { name: "Oculus Forge", slogan: "Forging new realities through vision." },
    CompanyPresetStatic { name: "Pinnacle Pixel", slogan: "Crafting visual excellence, one pixel at a time." },
    CompanyPresetStatic { name: "Reflex Byte", slogan: "Instant solutions, intuitive design." },
    CompanyPresetStatic { name: "Solstice Software", slogan: "Guiding you through the longest days of development." },
    CompanyPresetStatic { name: "Tesseract Innovations", slogan: "Unlocking dimensions of possibility." },
    CompanyPresetStatic { name: "Vanguard Logic", slogan: "Leading the charge in logical solutions." },
    CompanyPresetStatic { name: "Zenith Code", slogan: "The peak of programming perfection." },
    CompanyPresetStatic { name: "Binary Blossom", slogan: "Where ideas bloom into code." },
    CompanyPresetStatic { name: "Circuit Bloom", slogan: "Cultivating circuits, harvesting innovation." },
    CompanyPresetStatic { name: "Data Deluge", slogan: "Making waves in big data." },
    CompanyPresetStatic { name: "Encrypted Ember", slogan: "Secure by design, glowing with potential." },
    CompanyPresetStatic { name: "Flowstate Foundry", slogan: "Where focus meets function." },
    CompanyPresetStatic { name: "Glitch & Grind", slogan: "Embracing imperfections, refining solutions." },
    CompanyPresetStatic { name: "Haptic Pulse", slogan: "Feeling the rhythm of innovation." },
    CompanyPresetStatic { name: "Inertia Labs", slogan: "Overcoming obstacles, building momentum." },
    CompanyPresetStatic { name: "Jigsaw Junction", slogan: "Connecting the pieces of your digital puzzle." },
    CompanyPresetStatic { name: "Krypton Core", slogan: "The foundational element of robust software." },
    CompanyPresetStatic { name: "Loopback Logic", slogan: "Refining and perfecting through iteration." },
    CompanyPresetStatic { name: "Modulus Minds", slogan: "Finding the remainder of brilliance." },
    CompanyPresetStatic { name: "Nadir Navigators", slogan: "Guiding you from the lowest point to success." },
    CompanyPresetStatic { name: "Oracle Output", slogan: "Delivering prophetic solutions." },
    CompanyPresetStatic { name: "Paradigm Shift Co.", slogan: "Changing the way you see tech." },
    CompanyPresetStatic { name: "Quasar Quintet", slogan: "Five times the power, five times the light." },
    CompanyPresetStatic { name: "Recursion Realm", slogan: "Solving problems, one loop at a time." },
    CompanyPresetStatic { name: "Sentinel Stack", slogan: "Guarding your data, building your future." },
    CompanyPresetStatic { name: "Tangent Technologies", slogan: "Going off on brilliant new paths." },
    CompanyPresetStatic { name: "Ubiquitous Unit", slogan: "Present everywhere, working flawlessly." },
    CompanyPresetStatic { name: "Vortex Velocity", slogan: "Spinning up solutions at lightning speed." },
    CompanyPresetStatic { name: "Warp Drive Devs", slogan: "Faster than light development." },
    CompanyPresetStatic { name: "Xenon Xolutions", slogan: "Bright ideas for complex problems." },
    CompanyPresetStatic { name: "Yielding Yoctos", slogan: "Smallest ideas, biggest impact." },
    CompanyPresetStatic { name: "Zephyr Zystemz", slogan: "Lightweight solutions, heavy impact." },
    CompanyPresetStatic { name: "Ctrl-Alt-Defeat", slogan: "We fix your computer's bad habits." },
    CompanyPresetStatic { name: "Bit Fiddlers Union", slogan: "Finely tuning every bit." },
    CompanyPresetStatic { name: "Ramen & Regrets", slogan: "Fueling innovation with cheap noodles and late nights." },
    CompanyPresetStatic { name: "Compile & Chill", slogan: "Because coding should be relaxing... sometimes." },
    CompanyPresetStatic { name: "Syntax Error Squad", slogan: "Finding your mistakes so you don't have to." },
    CompanyPresetStatic { name: "Infinite Coffee Cup", slogan: "Our code runs on caffeine and dreams." },
    CompanyPresetStatic { name: "The Uncaught Exception", slogan: "Unexpectedly brilliant results." },
    CompanyPresetStatic { name: "Floating Point Friends", slogan: "Almost perfectly accurate, always." },
    CompanyPresetStatic { name: "The Agile Alligators", slogan: "Snappy development, devouring deadlines." },
    CompanyPresetStatic { name: "Spaghetti Code Solutions", slogan: "It works, don't touch it." },
    CompanyPresetStatic { name: "Legacy System Lovers", slogan: "We appreciate the classics, even if they're dusty." },
    CompanyPresetStatic { name: "The Bug Bounty Hunters", slogan: "We find 'em, we squash 'em." },
    CompanyPresetStatic { name: "Ctrl-Z Consulting", slogan: "Undoing your worst decisions, professionally." },
    CompanyPresetStatic { name: "Pixelated Perfectionists", slogan: "It's not blurry, it's impressionistic." },
    CompanyPresetStatic { name: "The Missing Semicolon", slogan: "Tiny details, huge impact." },
    CompanyPresetStatic { name: "Quantum Quandary", slogan: "Simultaneously right and wrong until observed." },
    CompanyPresetStatic { name: "Rubber Ducky Debugging Co.", slogan: "We listen to your problems, then solve them." },
    CompanyPresetStatic { name: "Ctrl+Shift+Escape Artists", slogan: "Breaking out of any problem." },
    CompanyPresetStatic { name: "The Recursive Raccoons", slogan: "Digging through garbage for gems." },
    CompanyPresetStatic { name: "Off-By-One Errors Inc.", slogan: "Almost there, every time." },
    CompanyPresetStatic { name: "The Merge Conflict Maestros", slogan: "Harmonizing your code." },
    CompanyPresetStatic { name: "Stack Overflow Survivors", slogan: "We've seen it all, and copied most of it." },
    CompanyPresetStatic { name: "The Binary Bard", slogan: "Composing code that sings." },
    CompanyPresetStatic { name: "Gigaflop Gurus", slogan: "Processing power beyond compare." },
    CompanyPresetStatic { name: "The Unhandled Promise", slogan: "Eventually, we deliver." },
    CompanyPresetStatic { name: "Cache Money Collective", slogan: "Speeding up your profits." },
    CompanyPresetStatic { name: "Zero-Day Dreamers", slogan: "Exploiting possibilities." },
    CompanyPresetStatic { name: "The Infinite Scroll", slogan: "Always more to explore." },
    CompanyPresetStatic { name: "The Backdoor Builders", slogan: "Secure access, always." },
    CompanyPresetStatic { name: "The Literal Lizards", slogan: "Taking your requests exactly as written." },
    CompanyPresetStatic { name: "The Glitch in the Matrix Co.", slogan: "Is it a bug, or is it fate?" },
    CompanyPresetStatic { name: "The Perpetual Beta", slogan: "Always improving, never finished." },
    CompanyPresetStatic { name: "The Syntax Shenanigans", slogan: "Playing with code, making magic." },
    CompanyPresetStatic { name: "The Dangling Pointer Partnership", slogan: "Holding onto potential." },
    CompanyPresetStatic { name: "The Byte-Sized Business", slogan: "Small solutions, big impact." },
    CompanyPresetStatic { name: "The Algorithm Alchemists", slogan: "Turning data into gold." },
    CompanyPresetStatic { name: "The Kernel Krew", slogan: "Deep down, we're the best." },
    CompanyPresetStatic { name: "The Network Ninjas", slogan: "Stealthy solutions, seamless connections." },
    CompanyPresetStatic { name: "The API Avengers", slogan: "Saving your integrations." },
    CompanyPresetStatic { name: "The Compiler Comrades", slogan: "Turning thoughts into executable reality." },
    CompanyPresetStatic { name: "The Debugging Dragons", slogan: "Breathing fire on your bugs." },
    CompanyPresetStatic { name: "The Firewall Friends", slogan: "Keeping you safe and sound." },
    CompanyPresetStatic { name: "The Open Source Operatives", slogan: "Collaborating for greatness." },
    CompanyPresetStatic { name: "The SaaS Squids", slogan: "Reaching out with flexible solutions." },
    CompanyPresetStatic { name: "The Blockchain Brawlers", slogan: "Fighting for decentralized futures." },
    CompanyPresetStatic { name: "The Cloud Comedians", slogan: "Making infrastructure funny." },
    CompanyPresetStatic { name: "The Data Dynasty", slogan: "Ruling the realm of information." },
    CompanyPresetStatic { name: "The Firmware Pharaohs", slogan: "Ancient tech, modern wisdom." },
    CompanyPresetStatic { name: "The Gigabyte Giants", slogan: "Massive solutions, tiny footprint." },
    CompanyPresetStatic { name: "The Hardware Hounds", slogan: "Sniffing out the best components." },
    CompanyPresetStatic { name: "The IoT Iguanas", slogan: "Connecting everything, sensing everything." },
    CompanyPresetStatic { name: "The JavaScript Jesters", slogan: "Making the web laugh with our code." },
    CompanyPresetStatic { name: "The Machine Learning Magicians", slogan: "Conjuring insights from data." },
    CompanyPresetStatic { name: "The Nanobyte Navigators", slogan: "Guiding you through the smallest details." },
    CompanyPresetStatic { name: "The Quantum Quokkas", slogan: "Happy code, happy customers." },
    CompanyPresetStatic { name: "The Server Sloths", slogan: "Slow and steady wins the race... sometimes." },
    CompanyPresetStatic { name: "The Webhook Wizards", slogan: "Automating your magic." },
    CompanyPresetStatic { name: "The YAML Yeti", slogan: "Legendary configuration, perfectly nested." },
    CompanyPresetStatic { name: "The Zip File Zookeepers", slogan: "Organizing your digital wild kingdom." },
    CompanyPresetStatic { name: "The Pixel Pirates", slogan: "Plundering possibilities, one pixel at a time." },
    CompanyPresetStatic { name: "The Database Dragons", slogan: "Guarding your precious data." },
    CompanyPresetStatic { name: "The Algorithm Avengers", slogan: "Bringing justice to complex problems." },
    CompanyPresetStatic { name: "The Code Whisperers", slogan: "Understanding what your software *really* wants." },
    CompanyPresetStatic { name: "The DevOp Dominators", slogan: "Seamless pipelines, undisputed reign." },
    CompanyPresetStatic { name: "The Frontend Falcons", slogan: "Soaring high with beautiful UIs." },
    CompanyPresetStatic { name: "The Backend Bears", slogan: "Strong, reliable, and always hibernating code." },
    CompanyPresetStatic { name: "The DevOps Dolphins", slogan: "Swimming gracefully through deployment." },
    CompanyPresetStatic { name: "The AI Alpacas", slogan: "Intelligent, fluffy, and surprisingly fast." },
    CompanyPresetStatic { name: "The Big Data Beavers", slogan: "Building massive data dams." },
    CompanyPresetStatic { name: "The Cybersecurity Squirrels", slogan: "Hoarding your security, burying your threats." },
    CompanyPresetStatic { name: "The UX Unicorns", slogan: "Magical user experiences, purely mythical satisfaction." },
    CompanyPresetStatic { name: "The VR Vultures", slogan: "Circling new realities, ready to swoop in." },
    CompanyPresetStatic { name: "The Web3 Walruses", slogan: "Chilling on the blockchain, tusking up solutions." },
    CompanyPresetStatic { name: "The AR Anteaters", slogan: "Sniffing out augmented realities." },
    CompanyPresetStatic { name: "The ML Moles", slogan: "Digging deep for data insights." },
    CompanyPresetStatic { name: "The Quantum Quokkas", slogan: "Happily ever after code." },
    CompanyPresetStatic { name: "The Algorithm Angels", slogan: "Divinely inspired code." },
    CompanyPresetStatic { name: "The Bitstream Butlers", slogan: "Serving your data needs with grace." },
    CompanyPresetStatic { name: "The Cybernetic Centaurs", slogan: "Half code, half horse, all power." },
    CompanyPresetStatic { name: "The Data Dynamo", slogan: "Unleashing the power of information." },
    CompanyPresetStatic { name: "The Electric Eels", slogan: "Shockingly good code." },
    CompanyPresetStatic { name: "The Giga-Gnome Group", slogan: "Small but mighty, big on performance." },
    CompanyPresetStatic { name: "The Hyperlink Hawks", slogan: "Soaring through the web." },
    CompanyPresetStatic { name: "The Ironclad Iterators", slogan: "Building strong, repeating success." },
    CompanyPresetStatic { name: "The Jitterbug Jammers", slogan: "Dancing through data, fixing latency." },
    CompanyPresetStatic { name: "The Kilo-Byte Knights", slogan: "Defending your digital kingdom." },
    CompanyPresetStatic { name: "The Logic Labyrinth", slogan: "Navigating complexity with ease." },
    CompanyPresetStatic { name: "The Mega-Mantis Minds", slogan: "Sharp, precise, and highly effective." },
    CompanyPresetStatic { name: "The Nano-Nexus Navigators", slogan: "Connecting the smallest points." },
    CompanyPresetStatic { name: "The Optimal Octopi", slogan: "Eight arms, infinite solutions." },
    CompanyPresetStatic { name: "The Protocol Pandas", slogan: "Following the rules, delivering results." },
    CompanyPresetStatic { name: "The Quibble Quenchers", slogan: "Resolving disputes, one line of code at a time." },
    CompanyPresetStatic { name: "The Router Rabbits", slogan: "Fast connections, hopping to it." },
    CompanyPresetStatic { name: "The Silicon Sirens", slogan: "Luring you with irresistible code." },
    CompanyPresetStatic { name: "The Tera-Titan Tech", slogan: "Massive solutions, built to last." },
    CompanyPresetStatic { name: "The Uplink Unicorns", slogan: "Magical connections, limitless potential." },
    CompanyPresetStatic { name: "The Vector Vipers", slogan: "Striking precisely with data." },
    CompanyPresetStatic { name: "The Widget Whales", slogan: "Big solutions for big problems." },
    CompanyPresetStatic { name: "The X-Ray Xperts", slogan: "Seeing through complexity." },
    CompanyPresetStatic { name: "The Yotta-Byte Yogis", slogan: "Calmly handling immense data." },
    CompanyPresetStatic { name: "The Zettabyte Zebras", slogan: "Uniquely patterned, perfectly processed." },
];
