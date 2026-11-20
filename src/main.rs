use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const STYLE: &str = include_str!("../assets/style.css");
const ADMIN_PASSWORD: &str = "GolpoOfMKBS.S@k1b";
const FIREBASE_URL: &str = "https://golpobela-c92f8-default-rtdb.asia-southeast1.firebasedatabase.app/stories.json";

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: usize,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub time: String,
    pub cover: String,
    pub color: String,
    pub excerpt: String,
    pub content: String,
}

fn initial_stories() -> Vec<Story> {
    vec![
        Story {
            id: 1,
            title: "নীলচিঠি".to_string(),
            author: "তানভীর আহমেদ".to_string(),
            genre: "রহস্য".to_string(),
            time: "৮ মিনিট".to_string(),
            cover: "✉".to_string(),
            color: "cover-blue".to_string(),
            excerpt: "বৃষ্টির রাতে পুরোনো ডাকবাক্সে একটা চিঠি পেল রোদেলা। খামের ওপর তার নাম, অথচ হাতের লেখাটা তার খুব চেনা—যে মানুষটা দশ বছর আগে হারিয়ে গিয়েছিল, ঠিক তার মতো।".to_string(),
            content: "বৃষ্টির রাতে পুরোনো ডাকবাক্সে একটা চিঠি পেল রোদেলা। খামের ওপর তার নাম, অথচ হাতের লেখাটা তার খুব চেনা—যে মানুষটা দশ বছর আগে হারিয়ে গিয়েছিল, ঠিক তার মতো।\n\nজানালার কাচে বৃষ্টির ফোঁটা অবিরাম শব্দ তুলে যাচ্ছিল। রোদেলা কাঁপাকাঁপা হাতে খামটা খুলল। ভেতরে ভাঁজ করা নীল কাগজ, আর পরিচিত সেই কালির গন্ধ। চিঠির প্রথম লাইনটা পড়েই তার বুকটা কেঁপে উঠল—\n\n'রোদেলা, আমি জানি তুমি ভাবছ আমি আর নেই। কিন্তু সময় সবসময় সরলরেখায় চলে না। যদি সত্য জানতে চাও, আজ মধ্যরাতে পুরোনো লাইব্রেরির পেছনের গেটে এসো।'\n\nবাইরে তখনো ঝোড়ো বাতাস বইছে। ঘড়িতে রাত এগারোটা বেজে চল্লিশ মিনিট। রোদেলা তার রেইনকোটটা গায়ে জড়িয়ে নিল। কিছু ডাক হয়তো উপেক্ষা করতে নেই।".to_string(),
        },
        Story {
            id: 2,
            title: "জোনাকির শহর".to_string(),
            author: "মেহের আফরোজ".to_string(),
            genre: "কল্পকাহিনি".to_string(),
            time: "১২ মিনিট".to_string(),
            cover: "✦".to_string(),
            color: "cover-purple".to_string(),
            excerpt: "শহরটা রাতে ঘুমায় না। কারণ প্রতিটি বাড়ির জানালায় জ্বলে ওঠে একটি করে জোনাকি, আর তারা সবাই মিলে পাহারা দেয় মানুষের স্বপ্ন।".to_string(),
            content: "শহরটা রাতে ঘুমায় না। কারণ প্রতিটি বাড়ির জানালায় জ্বলে ওঠে একটি করে জোনাকি, আর তারা সবাই মিলে পাহারা দেয় মানুষের স্বপ্ন।\n\nএখানে স্বপ্ন চুরি হওয়া এক ভয়াবহ অপরাধ। স্বপ্নচোরেরা রাতের আঁধারে ঘুরে বেড়ায়, মানুষের মনের সুন্দর স্মৃতিগুলো মুছে দিয়ে সেখানে বিষাদের মেঘ ছড়িয়ে দেয়।\n\nআরিশা ছিল এই শহরের জোনাকি-পালক। তার কাজ হলো প্রতি সন্ধ্যায় আকাশে হাজারো আলোকিত জোনাকি ছড়িয়ে দেওয়া। কিন্তু আজ রাতে একটা জোনাকির আলো নিভে গেল। শহরের সবচেয়ে প্রবীণ মানুষটির জানালা আজ অন্ধকার।\n\nআরিশা বুঝল, স্বপ্নচোর এবার শহরের সবচেয়ে পুরোনো স্মৃতিতে হাত দিয়েছে। সে তার লণ্ঠন হাতে নেমে পড়ল নিষিদ্ধ কুয়াশার গলিতে।".to_string(),
        },
        Story {
            id: 3,
            title: "শেষ ট্রেন".to_string(),
            author: "আরিফ রহমান".to_string(),
            genre: "রহস্য".to_string(),
            time: "৬ মিনিট".to_string(),
            cover: "↗".to_string(),
            color: "cover-orange".to_string(),
            excerpt: "স্টেশনে তখন কেউ নেই। শুধু দূরের সিগন্যালটা সবুজ হয়ে জ্বলে উঠল, আর কুয়াশার ভেতর থেকে ভেসে এল ট্রেনের হুইসেল।".to_string(),
            content: "স্টেশনে তখন কেউ নেই। শুধু দূরের সিগন্যালটা সবুজ হয়ে জ্বলে উঠল, আর কুয়াশার ভেতর থেকে ভেসে এল ট্রেনের হুইসেল।\n\nঘড়িতে রাত দুটো দশ। এই সময় কোনো ট্রেনের শিডিউল থাকার কথা নয়। প্ল্যাটফর্মের টিকিটমাস্টার বুড়ো রশীদ মিয়া চশমার ওপর দিয়ে তাকিয়ে দেখলেন, একটা কালো ইঞ্জিন ধীরগতিতে প্ল্যাটফর্মে এসে থামল।\n\nকামরাগুলো পুরো ফাঁকা, কিন্তু ভেতর থেকে ভেসে আসছে অদ্ভুত বাঁশির সুর। ট্রেনের শেষ বগি থেকে নেমে এলেন এক দীর্ঘদেহী ভদ্রলোক, হাতে কাঠের একটা ভারী বাক্স।\n\n'মাস্টার মশাই, ঘড়ির কাঁটা কি একটু থামিয়ে রাখা যায়?'—ভদ্রলোকের কণ্ঠ শুনে রশীদ মিয়ার গায়ের লোম খাড়া হয়ে উঠল। কারণ চল্লিশ বছর আগে এমন কণ্ঠের এক মানুষ এই প্ল্যাটফর্ম থেকেই নিখোঁজ হয়েছিলেন।".to_string(),
        },
        Story {
            id: 4,
            title: "কাঠের পুতুল".to_string(),
            author: "নওশীন কবির".to_string(),
            genre: "রোমাঞ্চ".to_string(),
            time: "১০ মিনিট".to_string(),
            cover: "♧".to_string(),
            color: "cover-green".to_string(),
            excerpt: "দাদুর ঘরের আলমারিতে রাখা পুতুলটার চোখ দুটো প্রতি রাতে একটু একটু করে অন্যদিকে ঘুরে থাকত।".to_string(),
            content: "দাদুর ঘরের আলমারিতে রাখা পুতুলটার চোখ দুটো প্রতি রাতে একটু একটু করে অন্যদিকে ঘুরে থাকত।\n\nতানিশা প্রথম ভেবেছিল হয়তো তার মনের ভুল। কিন্তু গতকাল সে কাচের গায়ে চক দিয়ে পুতুলের চোখের দৃষ্টির সোজাসুজি একটা দাগ টেনে রেখেছিল। আজ সকালে উঠে দেখে দাগের থেকে অন্তত তিন ইঞ্চি বাঁয়ে পুতুলটির চোখ নিবদ্ধ।\n\nসে আলমারির চাবি খুঁজে বের করল। কাঠের পুতুলটা হাতে নিতেই এক অদ্ভুত উষ্ণতা অনুভূত হলো—যেন কোনো প্রাণহীন কাঠে নয়, জীবন্ত কারো ত্বকে হাত দিয়েছে সে।\n\nপুতুলের পেছনে খোদাই করা ছিল একটি নাম এবং একটি তারিখ: 'মায়াবতী, ১৯২৩'। ঠিক সেই মুহূর্তে ঘরের বাতিটা নিভে গেল এবং কান্নার শব্দ পাওয়া গেল।".to_string(),
        },
        Story {
            id: 5,
            title: "শীতের রোদ".to_string(),
            author: "সাদিয়া ইসলাম".to_string(),
            genre: "ভালোবাসা".to_string(),
            time: "৫ মিনিট".to_string(),
            cover: "☼".to_string(),
            color: "cover-pink".to_string(),
            excerpt: "বারান্দার রোদে এক কাপ চা আর পুরোনো সেই মানুষটা—কিছু গল্পের শেষ হয় না, শুধু নতুন সকাল আসে।".to_string(),
            content: "বারান্দার রোদে এক কাপ চা আর পুরোনো সেই মানুষটা—কিছু গল্পের শেষ হয় না, শুধু নতুন সকাল আসে।\n\nকুয়াশা কাটতে শুরু করেছে। রোদ এসে পড়েছে তুলসী মঞ্চে। অদিতি চায়ের কাপে চুমুক দিয়ে বলল, 'তুমি কি মনে রেখেছ আজ কত বছর হলো?'\n\nনিলয় হাসল। চশমাটা টেবিলের ওপর রেখে বলল, 'যেদিন প্রথম রিকশায় ভিজেছিলাম, সেদিন থেকে হিসাব করলে পুরো পঁচিশ বছর চার মাস।'\n\nঅদিতি তাকিয়ে রইল নিলয়ের মুখের বলিরেখার দিকে। ভালোবাসা হয়তো প্রথম দেখার রোমাঞ্চে শুরু হয়, কিন্তু টিকে থাকে এই শান্ত রোদ আর বিশ্বস্ত এক কাপ চায়ের সাহচর্যে।".to_string(),
        },
        Story {
            id: 6,
            title: "বৃষ্টিভেজা সুর".to_string(),
            author: "অনন্যা রায়".to_string(),
            genre: "ভালোবাসা".to_string(),
            time: "৭ মিনিট".to_string(),
            cover: "♫".to_string(),
            color: "cover-blue".to_string(),
            excerpt: "পুরোনো সেতারে আবার সুর উঠল। যে সুরটা বহু বছর আগে ভুলে গিয়েছিল মেঘলা, আজ হঠাৎ করেই মনে পড়ে গেল।".to_string(),
            content: "পুরোনো সেতারে আবার সুর উঠল। যে সুরটা বহু বছর আগে ভুলে গিয়েছিল মেঘলা, আজ হঠাৎ করেই মনে পড়ে গেল।\n\nচিলেকোঠার জানালার পাশে বসে আঙুলগুলো যখন তারের ওপর খেলা করল, বৃষ্টির শব্দের সাথে মিলে তৈরি হলো এক অপার্থিব মেলোডি। দরজার সামনে এসে দাঁড়িয়েছিল সৈকত, তার ভেজা চোখে একরাশ নীরব কৃতজ্ঞতা।".to_string(),
        },
        Story {
            id: 7,
            title: "অন্ধকার কুটির".to_string(),
            author: "তাহমিদ হাসান".to_string(),
            genre: "হরর".to_string(),
            time: "৯ মিনিট".to_string(),
            cover: "🕯".to_string(),
            color: "cover-purple".to_string(),
            excerpt: "পাহাড়ে পথ হারিয়ে যে কুটিরে আশ্রয় নিল চার বন্ধু, সেখানকার দেওয়ালে টানানো ছিল তাদেরই ছবি! অথচ তারা এই পাহাড়ে আগে কখনো আসেনি।".to_string(),
            content: "পাহাড়ে পথ হারিয়ে যে কুটিরে আশ্রয় নিল চার বন্ধু, সেখানকার দেওয়ালে টানানো ছিল তাদেরই ছবি! অথচ তারা এই পাহাড়ে আগে কখনো আসেনি।\n\nকুটিরের মাঝখানে জ্বলছিল একটা নিঃসঙ্গ মোমবাতি। দেওয়ালে টাঙানো ফ্রেমগুলোর দিকে যতবারই তারা তাকাচ্ছিল, ছবির মানুষগুলোর মুখের অভিব্যক্তি যেন একটু একটু করে বদলে যাচ্ছিল।\n\nহঠাৎ বাইরে থেকে দরজায় কড়া নাড়ার ভারী আওয়াজ এল। চারজনই শক্ত হয়ে গেল—কারণ তাদের সাথে থাকা চতুর্থ বন্ধুটি বাইরে কোথাও নেই, অথচ দরজার ওপাশ থেকে তার কণ্ঠেই ভেসে এল, 'দরজাটা খোলো, বাইরে খুব ঠান্ডা!'".to_string(),
        },
    ]
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut stories = use_signal(initial_stories);
    let mut selected_genre = use_signal(|| "সব".to_string());
    let mut active_tab = use_signal(|| "হোম".to_string());
    let mut opened_story_id = use_signal(|| None::<usize>);
    let mut search_open = use_signal(|| false);
    let mut query = use_signal(|| String::new());

    // Firebase Sync Status
    let is_syncing = use_signal(|| false);
    let mut is_loading = use_signal(|| true);

    // Sidebar & Profile Modal State
    let mut menu_open = use_signal(|| false);
    let mut profile_open = use_signal(|| false);

    // Admin State
    let mut is_admin = use_signal(|| false);
    let mut show_password_modal = use_signal(|| false);
    let mut password_input = use_signal(|| String::new());
    let mut password_error = use_signal(|| false);
    let mut show_password_text = use_signal(|| false);

    // Drag and Drop reorder state
    let mut dragged_index = use_signal(|| None::<usize>);

    // Story Editor State (Add or Edit)
    let mut show_editor = use_signal(|| false);
    let editing_story_id = use_signal(|| None::<usize>);
    let mut form_title = use_signal(|| String::new());
    let mut form_author = use_signal(|| String::new());
    let mut form_genre = use_signal(|| "রহস্য".to_string());
    let mut form_time = use_signal(|| "৭ মিনিট".to_string());
    let mut form_cover = use_signal(|| "📖".to_string());
    let mut form_color = use_signal(|| "cover-blue".to_string());
    let mut form_excerpt = use_signal(|| String::new());
    let mut form_content = use_signal(|| String::new());
    let form_error = use_signal(|| String::new());

    let genres = ["সব", "রহস্য", "ভালোবাসা", "কল্পকাহিনি", "রোমাঞ্চ", "হরর"];

    // Automatically fetch stories from Firebase on initial launch
    use_effect(move || {
        spawn(async move {
            is_loading.set(true);
            if let Ok(resp) = reqwest::get(FIREBASE_URL).await {
                if let Ok(remote_stories) = resp.json::<Vec<Story>>().await {
                    if !remote_stories.is_empty() {
                        stories.set(remote_stories);
                    }
                }
            }
            is_loading.set(false);
        });
    });

    // Helper closure to save and sync stories to Firebase
    let sync_stories_to_firebase = {
        let mut stories = stories.clone();
        let mut is_syncing = is_syncing.clone();
        move |new_list: Vec<Story>| {
            stories.set(new_list.clone());
            spawn(async move {
                is_syncing.set(true);
                let client = reqwest::Client::new();
                let _ = client.put(FIREBASE_URL).json(&new_list).send().await;
                is_syncing.set(false);
            });
        }
    };

    // Helper closure to reorder stories (move from -> to)
    let move_story = {
        let mut sync_fn = sync_stories_to_firebase.clone();
        let stories = stories.clone();
        move |from_idx: usize, to_idx: usize| {
            let mut list = stories();
            if from_idx < list.len() && to_idx < list.len() && from_idx != to_idx {
                let item = list.remove(from_idx);
                list.insert(to_idx, item);
                sync_fn(list);
            }
        }
    };

    // Filter stories for home display
    let story_list = stories();
    let shown_stories: Vec<Story> = story_list
        .iter()
        .filter(|story| selected_genre() == "সব" || story.genre == selected_genre())
        .filter(|story| {
            query().is_empty()
                || story.title.contains(&query())
                || story.author.contains(&query())
                || story.excerpt.contains(&query())
        })
        .cloned()
        .collect();

    // Helper closure to open editor for new story
    let open_add_story = {
        let mut show_editor = show_editor.clone();
        let mut editing_story_id = editing_story_id.clone();
        let mut form_title = form_title.clone();
        let mut form_author = form_author.clone();
        let mut form_genre = form_genre.clone();
        let mut form_time = form_time.clone();
        let mut form_cover = form_cover.clone();
        let mut form_color = form_color.clone();
        let mut form_excerpt = form_excerpt.clone();
        let mut form_content = form_content.clone();
        let mut form_error = form_error.clone();
        move || {
            editing_story_id.set(None);
            form_title.set(String::new());
            form_author.set(String::new());
            form_genre.set("রহস্য".to_string());
            form_time.set("৭ মিনিট".to_string());
            form_cover.set("📖".to_string());
            form_color.set("cover-blue".to_string());
            form_excerpt.set(String::new());
            form_content.set(String::new());
            form_error.set(String::new());
            show_editor.set(true);
        }
    };

    // Helper closure to open editor for an existing story
    let open_edit_story = {
        let mut show_editor = show_editor.clone();
        let mut editing_story_id = editing_story_id.clone();
        let mut form_title = form_title.clone();
        let mut form_author = form_author.clone();
        let mut form_genre = form_genre.clone();
        let mut form_time = form_time.clone();
        let mut form_cover = form_cover.clone();
        let mut form_color = form_color.clone();
        let mut form_excerpt = form_excerpt.clone();
        let mut form_content = form_content.clone();
        let mut form_error = form_error.clone();
        let stories = stories.clone();
        move |story_id: usize| {
            if let Some(story) = stories().iter().find(|s| s.id == story_id) {
                editing_story_id.set(Some(story.id));
                form_title.set(story.title.clone());
                form_author.set(story.author.clone());
                form_genre.set(story.genre.clone());
                form_time.set(story.time.clone());
                form_cover.set(story.cover.clone());
                form_color.set(story.color.clone());
                form_excerpt.set(story.excerpt.clone());
                form_content.set(story.content.clone());
                form_error.set(String::new());
                show_editor.set(true);
            }
        }
    };

    // Helper closure to delete story
    let delete_story = {
        let mut sync_fn = sync_stories_to_firebase.clone();
        let stories = stories.clone();
        let mut opened_story_id = opened_story_id.clone();
        move |story_id: usize| {
            let mut list = stories();
            list.retain(|s| s.id != story_id);
            sync_fn(list);
            if opened_story_id() == Some(story_id) {
                opened_story_id.set(None);
            }
        }
    };

    // Find currently opened story if any
    let active_story = opened_story_id()
        .and_then(|id| stories().into_iter().find(|s| s.id == id));

    let section_heading = if selected_genre() == "সব" {
        "সকল গল্প".to_string()
    } else {
        format!("{} গল্প", selected_genre())
    };

    rsx! {
        document::Title { "গল্পবেলা — গল্পের আপন ঠিকানা" }
        document::Meta { name: "theme-color", content: "#fbf7f0" }
        document::Style { {STYLE} }

        div { class: "app-shell",
            // TOP BAR
            header { class: "topbar",
                div { class: "topbar-left",
                    button {
                        class: "menu-btn",
                        aria_label: "গল্পের তালিকা খুলুন",
                        onclick: move |_| menu_open.set(true),
                        span { class: "menu-icon", "☰" }
                    }
                    div {
                        class: "brand",
                        onclick: move |_| {
                            opened_story_id.set(None);
                            active_tab.set("হোম".into());
                        },
                        span { class: "brand-mark", "গ" }
                        span { class: "brand-text", "গল্পবেলা" }
                    }
                }
                div { class: "top-actions",
                    if is_syncing() {
                        span { class: "sync-indicator", title: "Firebase এ ডাটা সিঙ্ক হচ্ছে...", "🔄" }
                    }
                    button {
                        class: "icon-button",
                        aria_label: "খুঁজুন",
                        onclick: move |_| search_open.toggle(),
                        "⌕"
                    }
                    button {
                        class: if is_admin() { "avatar-button admin-active" } else { "avatar-button" },
                        aria_label: "প্রোফাইল ও সেটিংস",
                        onclick: move |_| profile_open.set(true),
                        span { class: "avatar-initial", if is_admin() { "★" } else { "র" } }
                    }
                }
            }

            // MAIN CONTENT VIEWS
            if let Some(story) = active_story {
                StoryReader {
                    story: story.clone(),
                    is_admin: is_admin(),
                    on_back: move |_| opened_story_id.set(None),
                    on_edit: {
                        let mut open_edit = open_edit_story.clone();
                        let story_id = story.id;
                        move |_| open_edit(story_id)
                    },
                }
            } else if active_tab() == "লাইব্রেরি" {
                main { class: "content library-page",
                    p { class: "eyebrow", "তোমার সংগ্রহ" }
                    h1 { class: "section-title", "লাইব্রেরি" }
                    p { class: "muted", "পরে পড়ার জন্য রাখা গল্পগুলো এখানে পাবে।" }
                    div { class: "story-list",
                        for story in stories().iter().take(3).cloned() {
                            StoryRow {
                                story: story.clone(),
                                is_admin: is_admin(),
                                on_open: move |id| opened_story_id.set(Some(id)),
                                on_edit: {
                                    let mut open_edit = open_edit_story.clone();
                                    move |id| open_edit(id)
                                },
                                on_delete: {
                                    let mut del = delete_story.clone();
                                    move |id| del(id)
                                },
                            }
                        }
                    }
                }
            } else if active_tab() == "আমার" {
                main { class: "content library-page",
                    p { class: "eyebrow", "তোমার গল্পের দুনিয়া" }
                    h1 { class: "section-title", "আমার পাঠ" }
                    p { class: "muted", "নিয়মিত গল্প পড়লে চিন্তার জগৎ প্রসারিত হয়।" }
                    div { class: "reading-card",
                        div { class: "reading-icon", "◷" }
                        div {
                            h3 { "পড়ার অভ্যাস" }
                            p { "প্রতিদিন অন্তত একটি করে ছোটগল্প পড়ুন।" }
                        }
                    }
                    div { class: "reading-stats",
                        div {
                            strong { "{stories().len()}" }
                            span { "উপলব্ধ গল্প" }
                        }
                        div {
                            strong { "২৫" }
                            span { "মিনিট পাঠ" }
                        }
                        div {
                            strong { if is_admin() { "অ্যাডমিন" } else { "পাঠক" } }
                            span { "স্ট্যাটাস" }
                        }
                    }
                    h2 { class: "profile-heading", "নির্বাচিত গল্প" }
                    if let Some(first_story) = stories().first().cloned() {
                        StoryRow {
                            story: first_story,
                            is_admin: is_admin(),
                            on_open: move |id| opened_story_id.set(Some(id)),
                            on_edit: {
                                let mut open_edit = open_edit_story.clone();
                                move |id| open_edit(id)
                            },
                            on_delete: {
                                let mut del = delete_story.clone();
                                move |id| del(id)
                            },
                        }
                    }
                }
            } else {
                main { class: "content",
                    if search_open() {
                        div { class: "search-box",
                            span { "⌕" }
                            input {
                                placeholder: "গল্পের শিরোনাম, লেখক বা বিষয় খুঁজুন...",
                                value: "{query}",
                                oninput: move |event| query.set(event.value()),
                            }
                            button {
                                class: "clear-search",
                                onclick: move |_| {
                                    query.set(String::new());
                                    search_open.set(false);
                                },
                                "×"
                            }
                        }
                    }

                    section { class: "welcome-row",
                        div {
                            p { class: "eyebrow", "বাংলা ছোটগল্পের সংগ্রহ · Firebase সিঙ্ক" }
                            h1 { "আজ কী পড়বেন?" }
                            p { class: "welcome-sub", "একটু গল্প, একটু নিজের সময়।" }
                        }
                        span { class: "sun-mark", "☼" }
                    }

                    if is_admin() {
                        div { class: "admin-quick-bar",
                            div { class: "admin-quick-info",
                                span { class: "admin-badge", "★ অ্যাডমিন মোড সক্রিয়" }
                                span { class: "admin-story-count", "মোট গল্প: {stories().len()}টি · Firebase সংযুক্ত" }
                            }
                            div { class: "admin-quick-buttons",
                                button {
                                    class: "btn-add-story",
                                    onclick: {
                                        let mut add = open_add_story.clone();
                                        move |_| add()
                                    },
                                    "＋ নতুন গল্প"
                                }
                                button {
                                    class: "btn-reorder-quick",
                                    onclick: move |_| profile_open.set(true),
                                    "↕ ক্রম সাজান"
                                }
                            }
                        }
                    }

                    // Featured story banner
                    if let Some(featured) = stories().first().cloned() {
                        section {
                            class: "featured",
                            onclick: {
                                let fid = featured.id;
                                move |_| opened_story_id.set(Some(fid))
                            },
                            div { class: "feature-copy",
                                span { class: "feature-label", "আজকের বিশেষ গল্প" }
                                h2 { "{featured.title}" }
                                p { "{featured.excerpt}" }
                                div { class: "feature-bottom",
                                    span { "{featured.author}" }
                                    span { "{featured.time}　↗" }
                                }
                            }
                            div { class: "feature-art",
                                div { class: "moon" }
                                div { class: "art-window" }
                                div { class: "art-book", "{featured.cover}" }
                                span { class: "art-star star-one", "✦" }
                                span { class: "art-star star-two", "✧" }
                            }
                        }
                    }

                    // Genre selector chips
                    section { class: "section-block",
                        div { class: "section-heading",
                            h2 { "বিভাগ অনুযায়ী" }
                            span { "মোট {shown_stories.len()}টি গল্প" }
                        }
                        div { class: "genre-strip",
                            for genre in genres {
                                button {
                                    class: if selected_genre() == genre { "genre-chip active" } else { "genre-chip" },
                                    onclick: move |_| selected_genre.set(genre.to_string()),
                                    "{genre}"
                                }
                            }
                        }
                    }

                    // Story list
                    section { class: "section-block latest-block",
                        div { class: "section-heading",
                            h2 { "{section_heading}" }
                            span { "পড়তে ট্যাপ করুন →" }
                        }
                        div { class: "story-list",
                            for story in shown_stories.iter().cloned() {
                                StoryRow {
                                    story: story.clone(),
                                    is_admin: is_admin(),
                                    on_open: move |id| opened_story_id.set(Some(id)),
                                    on_edit: {
                                        let mut open_edit = open_edit_story.clone();
                                        move |id| open_edit(id)
                                    },
                                    on_delete: {
                                        let mut del = delete_story.clone();
                                        move |id| del(id)
                                    },
                                }
                            }
                        }
                    }

                    if shown_stories.is_empty() {
                        div { class: "empty-state-box",
                            span { class: "empty-icon", "📭" }
                            p { class: "empty-state", "এই বিভাগে কোনো গল্প পাওয়া যায়নি। অন্য কোনো বিষয় খুঁজুন।" }
                        }
                    }
                }
            }

            // BOTTOM NAVIGATION
            nav { class: "bottom-nav",
                button {
                    class: if active_tab() == "হোম" && opened_story_id().is_none() { "nav-item active" } else { "nav-item" },
                    onclick: move |_| {
                        active_tab.set("হোম".into());
                        opened_story_id.set(None);
                    },
                    span { "⌂" }
                    span { "হোম" }
                }
                button {
                    class: if active_tab() == "লাইব্রেরি" && opened_story_id().is_none() { "nav-item active" } else { "nav-item" },
                    onclick: move |_| {
                        active_tab.set("লাইব্রেরি".into());
                        opened_story_id.set(None);
                    },
                    span { "▤" }
                    span { "লাইব্রেরি" }
                }
                button {
                    class: if active_tab() == "আমার" && opened_story_id().is_none() { "nav-item active" } else { "nav-item" },
                    onclick: move |_| {
                        active_tab.set("আমার".into());
                        opened_story_id.set(None);
                    },
                    span { "♡" }
                    span { "আমার" }
                }
            }

            // LEFT SIDEBAR (DRAWER) - ALL STORIES LIST
            if menu_open() {
                div {
                    class: "drawer-backdrop",
                    onclick: move |_| menu_open.set(false),
                }
                aside { class: "left-drawer",
                    div { class: "drawer-header",
                        div { class: "drawer-title-group",
                            span { class: "drawer-icon", "📚" }
                            div {
                                h2 { "গল্পের তালিকা" }
                                span { class: "drawer-count", "মোট {stories().len()}টি গল্প · ক্লাউড সিঙ্কড" }
                            }
                        }
                        button {
                            class: "drawer-close-btn",
                            aria_label: "বন্ধ করুন",
                            onclick: move |_| menu_open.set(false),
                            "×"
                        }
                    }

                    div { class: "drawer-content",
                        p { class: "drawer-subtitle", "পড়তে যেকোনো গল্প নির্বাচন করুন:" }
                        div { class: "drawer-stories-list",
                            for (idx, story) in stories().iter().enumerate() {
                                div {
                                    class: "drawer-story-item",
                                    onclick: {
                                        let sid = story.id;
                                        move |_| {
                                            opened_story_id.set(Some(sid));
                                            menu_open.set(false);
                                        }
                                    },
                                    div { class: "drawer-item-cover {story.color}",
                                        span { "{story.cover}" }
                                    }
                                    div { class: "drawer-item-details",
                                        div { class: "drawer-item-title-row",
                                            span { class: "drawer-item-num", "#{idx + 1}" }
                                            h4 { "{story.title}" }
                                        }
                                        p { class: "drawer-item-author", "লেখক: {story.author}" }
                                        div { class: "drawer-item-tags",
                                            span { class: "drawer-genre-tag", "{story.genre}" }
                                            span { class: "drawer-time-tag", "◷ {story.time}" }
                                        }
                                    }
                                    span { class: "drawer-item-arrow", "→" }
                                }
                            }
                        }
                    }

                    if is_admin() {
                        div { class: "drawer-footer",
                            button {
                                class: "drawer-add-btn",
                                onclick: {
                                    let mut add = open_add_story.clone();
                                    move |_| {
                                        menu_open.set(false);
                                        add();
                                    }
                                },
                                "＋ নতুন গল্প যোগ করুন"
                            }
                        }
                    }
                }
            }

            // RIGHT PROFILE MODAL / DRAWER
            if profile_open() {
                div {
                    class: "drawer-backdrop",
                    onclick: move |_| profile_open.set(false),
                }
                aside { class: "right-drawer",
                    div { class: "drawer-header",
                        div { class: "drawer-title-group",
                            span { class: "drawer-icon", "👤" }
                            h2 { "প্রোফাইল" }
                        }
                        button {
                            class: "drawer-close-btn",
                            aria_label: "বন্ধ করুন",
                            onclick: move |_| profile_open.set(false),
                            "×"
                        }
                    }

                    div { class: "profile-body",
                        // User Avatar Card
                        div { class: "profile-user-card",
                            div { class: if is_admin() { "profile-avatar admin" } else { "profile-avatar" },
                                if is_admin() { "★" } else { "র" }
                            }
                            h3 { class: "profile-user-name", if is_admin() { "অ্যাডমিনিস্ট্রেটর" } else { "প্রিয় পাঠক" } }
                            p { class: "profile-user-role",
                                if is_admin() {
                                    "গল্পবেলা অ্যাপ অ্যাডমিন (Firebase কন্ট্রোল)"
                                } else {
                                    "গল্পবেলা ক্লাউড রিডার"
                                }
                            }
                        }

                        // Stats Summary
                        div { class: "profile-stats-grid",
                            div { class: "profile-stat-box",
                                strong { "{stories().len()}" }
                                span { "মোট গল্প" }
                            }
                            div { class: "profile-stat-box",
                                strong { "Firebase" }
                                span { "ডাটাবেস" }
                            }
                            div { class: "profile-stat-box",
                                strong { if is_admin() { "Admin" } else { "Reader" } }
                                span { "ভূমিকা" }
                            }
                        }

                        // Admin Section
                        div { class: "admin-section-container",
                            h4 { class: "admin-section-heading", "অ্যাডমিন প্যানেল" }
                            if !is_admin() {
                                div { class: "admin-locked-card",
                                    p { "গল্প যোগ, পরিবর্তন, মুছে ফেলা ও ড্র্যাগ করে ক্রম সাজাতে অ্যাডমিন পাসওয়ার্ড প্রয়োজন।" }
                                    button {
                                        class: "btn-admin-login",
                                        onclick: move |_| {
                                            password_error.set(false);
                                            password_input.set(String::new());
                                            show_password_modal.set(true);
                                        },
                                        span { "🔒" }
                                        span { "অ্যাডমিন মোডে প্রবেশ করুন" }
                                    }
                                }
                            } else {
                                div { class: "admin-active-card",
                                    div { class: "admin-status-pill",
                                        span { "●" }
                                        span { "অ্যাডমিন সক্রিয় · Firebase সংযুক্ত" }
                                    }
                                    div { class: "admin-controls-grid",
                                        button {
                                            class: "btn-admin-action primary",
                                            onclick: {
                                                let mut add = open_add_story.clone();
                                                move |_| {
                                                    profile_open.set(false);
                                                    add();
                                                }
                                            },
                                            span { "＋" }
                                            span { "নতুন গল্প যোগ" }
                                        }
                                        button {
                                            class: "btn-admin-action logout",
                                            onclick: move |_| is_admin.set(false),
                                            span { "🚪" }
                                            span { "লগআউট" }
                                        }
                                    }

                                    div { class: "reorder-banner",
                                        span { class: "reorder-icon", "↕" }
                                        p { "গল্প ড্র্যাগ করুন (পিসিতে) অথবা ▲ / ▼ বাটনে ক্লিক করে সহজেই ক্রম সাজান। সাথে সাথে Firebase-এ সেভ হয়ে যাবে।" }
                                    }

                                    h5 { class: "manage-stories-title", "গল্পের ক্রম ও পরিচালনা ({stories().len()}টি)" }
                                    div { class: "admin-manage-list",
                                        for (idx, story) in stories().iter().cloned().enumerate() {
                                            div {
                                                class: if dragged_index() == Some(idx) { "admin-story-row dragging" } else { "admin-story-row" },
                                                draggable: "true",
                                                ondragstart: move |_| dragged_index.set(Some(idx)),
                                                ondragover: move |e| e.prevent_default(),
                                                ondrop: {
                                                    let mut move_fn = move_story.clone();
                                                    let mut dragged = dragged_index.clone();
                                                    move |_| {
                                                        if let Some(from) = dragged() {
                                                            move_fn(from, idx);
                                                            dragged.set(None);
                                                        }
                                                    }
                                                },
                                                div { class: "drag-handle", title: "ড্র্যাগ করে টানুন", "⋮⋮" }
                                                div { class: "admin-story-meta",
                                                    div { class: "admin-story-title-row",
                                                        span { class: "order-badge", "#{idx + 1}" }
                                                        strong { "{story.title}" }
                                                    }
                                                    span { "{story.author} · {story.genre}" }
                                                }
                                                div { class: "admin-order-controls",
                                                    button {
                                                        class: "admin-move-btn",
                                                        disabled: idx == 0,
                                                        title: "উপরে নিন",
                                                        onclick: {
                                                            let mut move_fn = move_story.clone();
                                                            move |_| {
                                                                if idx > 0 {
                                                                    move_fn(idx, idx - 1);
                                                                }
                                                            }
                                                        },
                                                        "▲"
                                                    }
                                                    button {
                                                        class: "admin-move-btn",
                                                        disabled: idx + 1 >= stories().len(),
                                                        title: "নিচে নিন",
                                                        onclick: {
                                                            let mut move_fn = move_story.clone();
                                                            move |_| {
                                                                if idx + 1 < stories().len() {
                                                                    move_fn(idx, idx + 1);
                                                                }
                                                            }
                                                        },
                                                        "▼"
                                                    }
                                                }
                                                div { class: "admin-story-btns",
                                                    button {
                                                        class: "admin-edit-mini-btn",
                                                        title: "সম্পাদনা করুন",
                                                        onclick: {
                                                            let mut open_edit = open_edit_story.clone();
                                                            let sid = story.id;
                                                            move |_| {
                                                                profile_open.set(false);
                                                                open_edit(sid);
                                                            }
                                                        },
                                                        "✎"
                                                    }
                                                    button {
                                                        class: "admin-del-mini-btn",
                                                        title: "মুছে ফেলুন",
                                                        onclick: {
                                                            let mut del = delete_story.clone();
                                                            let sid = story.id;
                                                            move |_| del(sid)
                                                        },
                                                        "🗑"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ADMIN PASSWORD MODAL
            if show_password_modal() {
                div {
                    class: "modal-backdrop",
                    onclick: move |_| show_password_modal.set(false),
                }
                div { class: "password-modal-dialog",
                    div { class: "modal-header",
                        div { class: "modal-title-wrap",
                            span { class: "modal-icon", "🔐" }
                            h3 { "অ্যাডমিন নিরাপত্তা যাচাই" }
                        }
                        button {
                            class: "modal-close-btn",
                            onclick: move |_| show_password_modal.set(false),
                            "×"
                        }
                    }
                    p { class: "modal-desc", "অ্যাডমিন অধিকার পেতে গোপন পাসওয়ার্ড প্রদান করুন:" }

                    div { class: "password-input-wrap",
                        input {
                            r#type: if show_password_text() { "text" } else { "password" },
                            placeholder: "পাসওয়ার্ড লিখুন...",
                            value: "{password_input}",
                            oninput: move |e| {
                                password_input.set(e.value());
                                password_error.set(false);
                            },
                        }
                        button {
                            class: "toggle-pass-btn",
                            r#type: "button",
                            onclick: move |_| show_password_text.toggle(),
                            if show_password_text() { "🙈" } else { "👁" }
                        }
                    }

                    if password_error() {
                        div { class: "error-alert",
                            span { "⚠️" }
                            span { "পাসওয়ার্ড সঠিক নয়! সঠিক পাসওয়ার্ড দিয়ে আবার চেষ্টা করুন।" }
                        }
                    }

                    div { class: "modal-actions",
                        button {
                            class: "btn-secondary",
                            onclick: move |_| show_password_modal.set(false),
                            "বাতিল"
                        }
                        button {
                            class: "btn-primary",
                            onclick: move |_| {
                                if password_input().trim() == ADMIN_PASSWORD {
                                    is_admin.set(true);
                                    show_password_modal.set(false);
                                    password_input.set(String::new());
                                    password_error.set(false);
                                } else {
                                    password_error.set(true);
                                }
                            },
                            "প্রবেশ করুন"
                        }
                    }
                }
            }

            // STORY EDITOR MODAL (ADD / MODIFY STORY)
            if show_editor() {
                div {
                    class: "modal-backdrop",
                    onclick: move |_| show_editor.set(false),
                }
                div { class: "editor-modal-dialog",
                    div { class: "modal-header",
                        div { class: "modal-title-wrap",
                            span { class: "modal-icon", if editing_story_id().is_some() { "✎" } else { "＋" } }
                            h3 {
                                if editing_story_id().is_some() {
                                    "গল্প সম্পাদনা করুন"
                                } else {
                                    "নতুন গল্প যোগ করুন"
                                }
                            }
                        }
                        button {
                            class: "modal-close-btn",
                            onclick: move |_| show_editor.set(false),
                            "×"
                        }
                    }

                    div { class: "editor-form-scroll",
                        if !form_error().is_empty() {
                            div { class: "error-alert",
                                span { "⚠️" }
                                span { "{form_error}" }
                            }
                        }

                        div { class: "form-group",
                            label { "গল্পের শিরোনাম *" }
                            input {
                                placeholder: "যেমন: নীলচিঠি, শেষ ট্রেন...",
                                value: "{form_title}",
                                oninput: move |e| form_title.set(e.value()),
                            }
                        }

                        div { class: "form-row",
                            div { class: "form-group flex-1",
                                label { "লেখকের নাম *" }
                                input {
                                    placeholder: "লেখকের নাম লিখুন...",
                                    value: "{form_author}",
                                    oninput: move |e| form_author.set(e.value()),
                                }
                            }
                            div { class: "form-group flex-1",
                                label { "পড়ার সময়" }
                                input {
                                    placeholder: "যেমন: ৮ মিনিট",
                                    value: "{form_time}",
                                    oninput: move |e| form_time.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-row",
                            div { class: "form-group flex-1",
                                label { "গল্পের বিভাগ (জঁর)" }
                                select {
                                    value: "{form_genre}",
                                    onchange: move |e| form_genre.set(e.value()),
                                    option { value: "রহস্য", "রহস্য" }
                                    option { value: "ভালোবাসা", "ভালোবাসা" }
                                    option { value: "কল্পকাহিনি", "কল্পকাহিনি" }
                                    option { value: "রোমাঞ্চ", "রোমাঞ্চ" }
                                    option { value: "হরর", "হরর" }
                                    option { value: "নাটক", "নাটক" }
                                }
                            }
                            div { class: "form-group flex-1",
                                label { "কভার আইকন / ইমোজি" }
                                select {
                                    value: "{form_cover}",
                                    onchange: move |e| form_cover.set(e.value()),
                                    option { value: "📖", "📖 বই" }
                                    option { value: "✉", "✉ চিঠি" }
                                    option { value: "✦", "✦ তারা" }
                                    option { value: "↗", "↗ গতি" }
                                    option { value: "♧", "♧ পুতুল" }
                                    option { value: "☼", "☼ সূর্য" }
                                    option { value: "♫", "♫ সুর" }
                                    option { value: "🕯", "🕯 মোমবাতি" }
                                    option { value: "🌙", "🌙 চাঁদ" }
                                    option { value: "☕", "☕ চা" }
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "কভারের থিম রঙ" }
                            div { class: "color-selector-row",
                                button {
                                    class: if form_color() == "cover-blue" { "color-choice-btn color-blue active" } else { "color-choice-btn color-blue" },
                                    r#type: "button",
                                    onclick: move |_| form_color.set("cover-blue".into()),
                                    "নীল"
                                }
                                button {
                                    class: if form_color() == "cover-purple" { "color-choice-btn color-purple active" } else { "color-choice-btn color-purple" },
                                    r#type: "button",
                                    onclick: move |_| form_color.set("cover-purple".into()),
                                    "বেগুনি"
                                }
                                button {
                                    class: if form_color() == "cover-orange" { "color-choice-btn color-orange active" } else { "color-choice-btn color-orange" },
                                    r#type: "button",
                                    onclick: move |_| form_color.set("cover-orange".into()),
                                    "কমলা"
                                }
                                button {
                                    class: if form_color() == "cover-green" { "color-choice-btn color-green active" } else { "color-choice-btn color-green" },
                                    r#type: "button",
                                    onclick: move |_| form_color.set("cover-green".into()),
                                    "সবুজ"
                                }
                                button {
                                    class: if form_color() == "cover-pink" { "color-choice-btn color-pink active" } else { "color-choice-btn color-pink" },
                                    r#type: "button",
                                    onclick: move |_| form_color.set("cover-pink".into()),
                                    "গোলাপি"
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "এক নজরে সংক্ষেপ (Excerpt) *" }
                            textarea {
                                rows: 2,
                                placeholder: "গল্পের সংক্ষেপ ১-২ লাইনে লিখুন...",
                                value: "{form_excerpt}",
                                oninput: move |e| form_excerpt.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "সম্পূর্ণ গল্পের বিবরণ (Full Story) *" }
                            textarea {
                                rows: 8,
                                placeholder: "সম্পূর্ণ গল্প এখানে লিখুন...",
                                value: "{form_content}",
                                oninput: move |e| form_content.set(e.value()),
                            }
                        }
                    }

                    div { class: "modal-actions",
                        button {
                            class: "btn-secondary",
                            onclick: move |_| show_editor.set(false),
                            "বাতিল"
                        }
                        button {
                            class: "btn-primary",
                            onclick: {
                                let mut sync_fn = sync_stories_to_firebase.clone();
                                let stories = stories.clone();
                                let mut show_editor = show_editor.clone();
                                let mut form_error = form_error.clone();
                                let editing_id = editing_story_id();
                                let title = form_title();
                                let author = form_author();
                                let genre = form_genre();
                                let time = form_time();
                                let cover = form_cover();
                                let color = form_color();
                                let excerpt = form_excerpt();
                                let content = form_content();

                                move |_| {
                                    if title.trim().is_empty() {
                                        form_error.set("অনুগ্রহ করে গল্পের শিরোনাম লিখুন!".to_string());
                                        return;
                                    }
                                    if author.trim().is_empty() {
                                        form_error.set("অনুগ্রহ করে লেখকের নাম লিখুন!".to_string());
                                        return;
                                    }
                                    if excerpt.trim().is_empty() {
                                        form_error.set("অনুগ্রহ করে গল্পের সংক্ষেপ লিখুন!".to_string());
                                        return;
                                    }
                                    if content.trim().is_empty() {
                                        form_error.set("অনুগ্রহ করে গল্পের বিষয়বস্তু লিখুন!".to_string());
                                        return;
                                    }

                                    let mut list = stories();
                                    if let Some(target_id) = editing_id {
                                        // Update existing story
                                        if let Some(s) = list.iter_mut().find(|item| item.id == target_id) {
                                            s.title = title.clone();
                                            s.author = author.clone();
                                            s.genre = genre.clone();
                                            s.time = time.clone();
                                            s.cover = cover.clone();
                                            s.color = color.clone();
                                            s.excerpt = excerpt.clone();
                                            s.content = content.clone();
                                        }
                                    } else {
                                        // Insert new story
                                        let next_id = list.iter().map(|s| s.id).max().unwrap_or(0) + 1;
                                        list.push(Story {
                                            id: next_id,
                                            title: title.clone(),
                                            author: author.clone(),
                                            genre: genre.clone(),
                                            time: if time.trim().is_empty() { "৭ মিনিট".into() } else { time.clone() },
                                            cover: cover.clone(),
                                            color: color.clone(),
                                            excerpt: excerpt.clone(),
                                            content: content.clone(),
                                        });
                                    }
                                    sync_fn(list);
                                    show_editor.set(false);
                                }
                            },
                            if editing_story_id().is_some() { "পরিবর্তন সংরক্ষণ করুন" } else { "গল্প যোগ করুন" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StoryReader(
    story: Story,
    is_admin: bool,
    on_back: EventHandler<()>,
    on_edit: EventHandler<()>,
) -> Element {
    let mut is_read = use_signal(|| false);

    // Split content by paragraphs
    let paragraphs: Vec<&str> = story.content.split("\n\n").collect();

    rsx! {
        main { class: "reader-page",
            div { class: "reader-top-nav",
                button {
                    class: "back-button",
                    onclick: move |_| on_back.call(()),
                    "←　ফিরে যান"
                }
                if is_admin {
                    button {
                        class: "reader-edit-btn",
                        onclick: move |_| on_edit.call(()),
                        "✎ এই গল্প সম্পাদনা"
                    }
                }
            }
            div { class: "reader-cover {story.color}",
                span { "{story.cover}" }
            }
            div { class: "reader-meta", "{story.genre}　·　{story.time} পাঠ" }
            h1 { class: "reader-title", "{story.title}" }
            p { class: "reader-author", "লিখেছেন {story.author}" }
            div { class: "reader-rule" }

            for para in paragraphs {
                p { class: "story-text", "{para}" }
            }

            button {
                class: if is_read() { "read-continue completed" } else { "read-continue" },
                onclick: move |_| is_read.toggle(),
                if is_read() { "✓　সম্পূর্ণ পড়া হয়েছে" } else { "পড়া শেষ হয়েছে হিসেবে চিহ্নিত করুন　✓" }
            }
        }
    }
}

#[component]
fn StoryRow(
    story: Story,
    is_admin: bool,
    on_open: EventHandler<usize>,
    on_edit: EventHandler<usize>,
    on_delete: EventHandler<usize>,
) -> Element {
    let mut is_saved = use_signal(|| false);
    let story_id = story.id;

    rsx! {
        article {
            class: "story-row",
            onclick: move |_| on_open.call(story_id),
            div { class: "story-cover {story.color}",
                span { "{story.cover}" }
            }
            div { class: "story-info",
                span { class: "story-genre", "{story.genre}" }
                h3 { "{story.title}" }
                p { "{story.author}" }
                span { class: "story-duration", "◷　{story.time}" }
            }
            div { class: "story-row-actions",
                if is_admin {
                    button {
                        class: "admin-inline-btn edit",
                        title: "সম্পাদনা",
                        aria_label: "সম্পাদনা",
                        onclick: move |e| {
                            e.stop_propagation();
                            on_edit.call(story_id);
                        },
                        "✎"
                    }
                    button {
                        class: "admin-inline-btn delete",
                        title: "মুছে ফেলুন",
                        aria_label: "মুছে ফেলুন",
                        onclick: move |e| {
                            e.stop_propagation();
                            on_delete.call(story_id);
                        },
                        "🗑"
                    }
                }
                button {
                    class: if is_saved() { "bookmark saved" } else { "bookmark" },
                    aria_label: "সংরক্ষণ করুন",
                    onclick: move |event| {
                        event.stop_propagation();
                        is_saved.toggle();
                    },
                    if is_saved() { "♥" } else { "♡" }
                }
            }
        }
    }
}
