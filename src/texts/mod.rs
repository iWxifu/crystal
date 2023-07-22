use rand::Rng;

static PISSED_STRINGS: &[&str] = &[
    "kys",
    "god damn i'm not surprised your dad left",
    "I'd rather be 14 and an alcoholic than talk to you ever again",
    "after looking at you democracy seems like a bad idea",
    "this isn't roblox chat, grow up you virgin",
    "aww did ur mc gf break up again?",
    "please stfu, you'd prolly fuck a turtle",
    "go jerk off to nick wilde",
    "oh no you're on your emo arc",
    "aww calm down before going wolf mode 🥺",
    "ok scum",
    "anyone heard anything?",
    "you seem like you'd unironically search a discord kitten",
    "you seem like you listen to pop music",
    "mental health PSA: remember to cut longways down, not sideways",
    "i'd call you a tard but that'd be an insult to all the other ones",
    "are you always this dumb?",
    "god damn i'm surprised you even found a tit to suck on",
    "fag\nfag\nfag\nfag\nfag\nfag\nfag\nfag\nfag\nfag"
];

static CALLED_FOR: &[&str] = &[
    "what",
    "the fuck do you want",
    "ew its you",
    "leave me alone bitch",
    "aww what do you need? help? a mom? grow up.",
    "stfu im watching netflix",
    "bruh let me sleep",
    "..yes?",
    "wut?",
    "heheheh :3",
    "just a second",
    "FUCK OFF WHORE",
    "yes hun?",
    "IT WASN'T ME",
    "yes love?",
    "what's it cutie?",
    "babe?",
];

static ROBLOX_STRINGS: &[&str] = &[
    "virgin",
    "you seem like a roblox player",
    "aww go back to watching roblox porn",
    "how's ur adopt me gf?",
    "get ur moms credit card for bobux, clown"
];

static MUSIC_STRINGS: &[&str] = &[
    "no I won't listen to ur sex mixtape",
    "let me guess, basic ass euro pop from a basic ass bitch?",
    "damn, your music taste could have saved you but it... didn't.",
    "what fucking language is that?",
    "awww I listen to unknown artists I'm so special look at me",
    "thank god I don't ever have to get in a car with u",
    "spotify should ban your account holy shit",
    "yeah no I'd rather be deaf"
];

static VSCO_STRINGS: &[&str] = &[
    "aww stfu you're not cute",
    "bitch you aren't acting cute but stupid",
    "shut the fuck up bitch",
    "pathetic anime hoe",
    "basic ass anime bitch",
    "i'm literally gonna slap you",
    "istg i'm gonna beat you harder than your dad ever could",
    "stop acting cute before my hand ends up in your face"
];

pub fn get_toxic() -> String {
    let index = rand::thread_rng().gen_range(0..PISSED_STRINGS.len());
    PISSED_STRINGS[index].to_string()
}

pub fn get_music() -> String {
    let index = rand::thread_rng().gen_range(0..MUSIC_STRINGS.len());
    MUSIC_STRINGS[index].to_string()
}