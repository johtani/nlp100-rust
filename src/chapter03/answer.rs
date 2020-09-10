use flate2::read::GzDecoder;
use regex::Regex;
use reqwest::StatusCode;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Deserialize)]
pub struct Article {
    title: String,
    text: String,
}

impl Article {
    pub fn lines_from_text(&self) -> Vec<String> {
        self.text.split("\n").map(|line| line.to_string()).collect()
    }
}

// https://docs.rs/flate2/1.0.14/flate2/read/struct.GzDecoder.html
pub fn extract_ndjson_from_gzip(input_file_name: &str) -> Vec<String> {
    let f = File::open(input_file_name).expect("file not found?");
    let gz = GzDecoder::new(f);
    let buf = BufReader::new(gz);
    let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    return lines;
}

// ch03-20. JSONデータの読み込み
// https://serde.rs/
pub fn load_json(input_file_name: &str, target_title: &str) -> Vec<Article> {
    let mut results = vec![];
    let ndjson = extract_ndjson_from_gzip(input_file_name);
    for json in ndjson {
        let article: Article = serde_json::from_str(json.as_str()).expect("json parse error");
        if article.title == target_title {
            results.push(article);
        }
    }
    return results;
}

// ch03-21. カテゴリ名を含む行を抽出
pub fn extract_category_lines(article: &Article) -> Vec<String> {
    let re = Regex::new(r"\[\[Category:(.*)\]\]").expect("syntax error in regex");
    let mut lines = vec![];
    article.lines_from_text().iter().for_each(|line| {
        if re.is_match(line) {
            lines.push(line.to_string());
        }
    });
    return lines;
}

// ch03-22. カテゴリ名の抽出
pub fn extract_categories(article: &Article) -> Vec<String> {
    let re = Regex::new(r"\[\[Category:([^\|]+)\|?.*\]\]").expect("syntax error in regex");
    let mut categories = vec![];
    article.lines_from_text().iter().for_each(|line| {
        for cap in re.captures_iter(line) {
            categories.push(cap[1].to_string());
        }
    });
    return categories;
}

// ch03-23. セクション構造
pub struct Section {
    section: String,
    level: u8,
}
pub fn extract_sections(article: &Article) -> Vec<Section> {
    let re = Regex::new(r"(={2,})([^=]+)(={2,})").expect("syntax error in regex");
    let mut sections: Vec<Section> = vec![];
    article.lines_from_text().iter().for_each(|line| {
        for cap in re.captures_iter(line) {
            let section = Section {
                section: cap[2].to_string(),
                level: cap[1].len() as u8 - 1,
            };
            sections.push(section);
        }
    });
    return sections;
}

// ch03-24. ファイル参照の抽出
pub fn extract_files(article: &Article) -> Vec<String> {
    let re = Regex::new(r"\[\[ファイル:([^\|]+)(?:\|+|\]\])").expect("syntax error in regex");
    let mut files: Vec<String> = vec![];
    article.lines_from_text().iter().for_each(|line| {
        for cap in re.captures_iter(line) {
            let file = cap[1].to_string();
            files.push(file);
        }
    });
    return files;
}

// ch03-25. テンプレートの抽出
pub fn extract_basic_info<T: Cleaner>(article: &Article, cleaner: T) -> HashMap<String, String> {
    let re = Regex::new(r"(?ms)(?:^\{\{基礎情報.+?$)(.+?)(?:^}}$)").expect("syntax error in regex");
    let mut basic_info: HashMap<String, String> = HashMap::new();
    for cap in re.captures_iter(article.text.as_str()) {
        let basic_info_str = cap[1].to_string();
        let entries: Vec<String> = basic_info_str.split("\n|").map(|e| e.to_string()).collect();
        let re_entry = Regex::new(r"(?ms)(.+?)(?:[\s]*)=(?:[\s]*)(.+?)\z")
            .expect("syntax error in regexp for entry");
        for entry in entries {
            if entry.trim().len() > 0 {
                for cap_entry in re_entry.captures_iter(entry.trim()) {
                    basic_info.insert(
                        cap_entry[1].to_string(),
                        cleaner.remove_markup(cap_entry[2].as_ref()),
                    );
                }
            }
        }
    }
    //
    return basic_info;
}

pub trait Cleaner {
    fn remove_markup(&self, original: &str) -> String;
}

struct NoneCleaner {}

impl Cleaner for NoneCleaner {
    fn remove_markup(&self, original: &str) -> String {
        return original.to_string();
    }
}

// ch03-26. 強調マークアップの除去
struct StrongCleaner {}

impl Cleaner for StrongCleaner {
    fn remove_markup(&self, original: &str) -> String {
        let re = Regex::new(r"('{2,5})").expect("syntax error in regex");
        return re.replace_all(original, "").to_string();
    }
}

// ch03-27. 内部リンクの除去
struct LinkCleaner {
    chain: StrongCleaner,
}

impl Cleaner for LinkCleaner {
    fn remove_markup(&self, original: &str) -> String {
        let no_strong = self.chain.remove_markup(original);
        if no_strong.contains("[[") {
            let mut text = String::from("");
            let re = Regex::new(r"(?:\[\[)(?P<link>.+?)(?:\]\])").expect("syntax error in regex");
            text.push_str(re.replace_all(no_strong.as_str(), "$link").as_ref());
            return text;
        } else {
            return no_strong;
        }
    }
}

// ch03-28. MediaWikiマークアップの除去
//TODO...

// ch03-29. 国旗画像のURLを取得する
pub fn get_country_flag_url(basic_info: HashMap<String, String>) -> Option<String> {
    return match basic_info.get("国旗画像") {
        Some(file_name) => Some(get_image_url(file_name)),
        None => None,
    };
}

fn get_image_url(file_name: &str) -> String {
    let mut _rt = tokio::runtime::Runtime::new().expect("Fail initializing runtime");
    let task = call_api(file_name);
    _rt.block_on(task).expect("Something wrong...")
}

#[derive(Debug, Deserialize)]
pub struct MediaWikiResponse {
    query: Value,
}

impl MediaWikiResponse {
    pub fn get_url(&self) -> Option<String> {
        let pages = self.query.as_object()?.get("pages")?;
        let key = pages.as_object().unwrap().keys().next()?;
        return Some(String::from(
            pages
                .get(key)?
                .as_object()?
                .get("imageinfo")?
                .as_array()?
                .first()?
                .as_object()
                .unwrap()
                .get("url")?
                .as_str()?,
        ));
    }
}

async fn call_api(file_name: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let file_name2 = format!("File:{}", file_name);
    //let mut file_name2 = file_name.to_string().replace(" ", "_");
    let query = [
        ("action", "query"),
        ("format", "json"),
        ("prop", "imageinfo"),
        ("iiprop", "url"),
        ("titles", file_name2.as_str()),
    ];
    let result = client
        .get("https://en.wikipedia.org/w/api.php")
        .query(&query)
        .send()
        .await;

    match result {
        Ok(response) => match response.status() {
            StatusCode::OK => {
                let body = response.json::<MediaWikiResponse>().await;
                match body {
                    Ok(obj) => match obj.get_url() {
                        Some(url) => Ok(url),
                        None => Err(String::from("Cannot get url...")),
                    },
                    Err(error) => Err(error.to_string()),
                }
            }
            _ => Err(String::from(format!(
                "Status code is {}.",
                response.status()
            ))),
        },
        Err(error) => {
            let error_msg = format!("Error occurred... {:?}", error);
            println!("{}", error_msg.as_str());
            Err(error_msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter03::answer::{
        extract_basic_info, extract_categories, extract_category_lines, extract_files,
        extract_ndjson_from_gzip, extract_sections, get_country_flag_url, load_json, LinkCleaner,
        NoneCleaner, Section, StrongCleaner,
    };

    const INPUT_PATH: &str = "data/jawiki-country.json.gz";
    const KEYWORD: &str = "イギリス";

    #[test]
    pub fn success_extract_ndjson_from_gzip() {
        let lines = extract_ndjson_from_gzip(INPUT_PATH);
        //        assert_eq!();
        assert_eq!(lines.len(), 248);
    }

    #[test]
    pub fn success_20_load_json() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        assert_eq!(1, articles.len());
        assert_eq!(KEYWORD, articles[0].title);
        println!("{}", articles[0].text);
    }

    #[test]
    pub fn success_21_extract_category_lines() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        let article = articles.get(0);
        let expected_lines = vec![
            "[[Category:イギリス|*]]",
            "[[Category:イギリス連邦加盟国]]",
            "[[Category:英連邦王国|*]]",
            "[[Category:G8加盟国]]",
            "[[Category:欧州連合加盟国|元]]",
            "[[Category:海洋国家]]",
            "[[Category:現存する君主国]]",
            "[[Category:島国]]",
            "[[Category:1801年に成立した国家・領域]]",
        ];

        match article {
            None => panic!("fail to load {} article", KEYWORD),
            Some(target) => {
                let category_lines = extract_category_lines(target);
                assert_eq!(expected_lines.len(), category_lines.len());
                for actual in category_lines {
                    assert_eq!(true, actual.contains("Category"));
                    assert!(expected_lines.contains(&actual.as_str()));
                }
            }
        }
    }

    #[test]
    pub fn success_22_extract_categories() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        let article = articles.get(0);
        let expected_lines = vec![
            "イギリス",
            "イギリス連邦加盟国",
            "英連邦王国",
            "G8加盟国",
            "欧州連合加盟国",
            "海洋国家",
            "現存する君主国",
            "島国",
            "1801年に成立した国家・領域",
        ];
        match article {
            None => panic!("fail to load {} article", KEYWORD),
            Some(target) => {
                let categories = extract_categories(target);
                assert_eq!(expected_lines.len(), categories.len());
                for actual in categories {
                    assert!(expected_lines.contains(&actual.as_str()));
                    assert!(
                        expected_lines.iter().any(|e| e == &actual.as_str()),
                        "{} fails",
                        actual
                    );
                }
            }
        }
    }

    fn expected_lines_23() -> Vec<(u8, &'static str)> {
        vec![
            (1, "国名"),
            (1, "歴史"),
            (1, "地理"),
            (2, "主要都市"),
            (2, "気候"),
            (1, "政治"),
            (2, "元首"),
            (2, "法"),
            (2, "内政"),
            (2, "地方行政区分"),
            (2, "外交・軍事"),
            (1, "経済"),
            (2, "鉱業"),
            (2, "農業"),
            (2, "貿易"),
            (2, "不動産"),
            (2, "エネルギー政策"),
            (2, "通貨"),
            (2, "企業"),
            (3, "通信"),
            (1, "交通"),
            (2, "道路"),
            (2, "鉄道"),
            (2, "海運"),
            (2, "航空"),
            (1, "科学技術"),
            (1, "国民"),
            (2, "言語"),
            (2, "宗教"),
            (2, "婚姻"),
            (2, "移住"),
            (2, "教育"),
            (2, "医療"),
            (1, "文化"),
            (2, "食文化"),
            (2, "文学"),
            (2, "哲学"),
            (2, "音楽"),
            (3, "ポピュラー音楽"),
            (2, "映画"),
            (2, "コメディ"),
            (2, "国花"),
            (2, "世界遺産"),
            (2, "祝祭日"),
            (2, "スポーツ"),
            (3, "サッカー"),
            (3, "クリケット"),
            (3, "競馬"),
            (3, "モータースポーツ"),
            (3, "野球"),
            (3, "カーリング"),
            (3, "自転車競技"),
            (1, "脚注"),
            (1, "関連項目"),
            (1, "外部リンク"),
        ]
    }

    #[test]
    pub fn success_23_extract_sections() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        let article = articles.get(0);
        let expected_lines = expected_lines_23();
        match article {
            None => panic!("fail to load {} article", KEYWORD),
            Some(target) => {
                let sections: Vec<Section> = extract_sections(target);
                assert_eq!(expected_lines.len(), sections.len());
                sections
                    .iter()
                    .zip(expected_lines)
                    .for_each(|(actual, expect)| {
                        assert_eq!(&expect.1, &actual.section.as_str().trim(), "{}", &expect.1);
                        assert_eq!(&expect.0, &actual.level, "{}", &expect.1);
                    });
            }
        }
    }

    fn expected_lines_24() -> Vec<&'static str> {
        vec![
            "Royal Coat of Arms of the United Kingdom.svg",
            "United States Navy Band - God Save the Queen.ogg",
            "Descriptio Prime Tabulae Europae.jpg",
            "Lenepveu, Jeanne d'Arc au siège d'Orléans.jpg",
            "London.bankofengland.arp.jpg",
            "Battle of Waterloo 1815.PNG",
            "Uk topo en.jpg",
            "BenNevis2005.jpg",
            "Population density UK 2011 census.png",
            "2019 Greenwich Peninsula & Canary Wharf.jpg",
            "Birmingham Skyline from Edgbaston Cricket Ground crop.jpg",
            "Leeds CBD at night.jpg",
            "Glasgow and the Clyde from the air (geograph 4665720).jpg",
            "Palace of Westminster, London - Feb 2007.jpg",
            "Scotland Parliament Holyrood.jpg",
            "Donald Trump and Theresa May (33998675310) (cropped).jpg",
            "Soldiers Trooping the Colour, 16th June 2007.jpg",
            "City of London skyline from London City Hall - Oct 2008.jpg",
            "Oil platform in the North SeaPros.jpg",
            "Eurostar at St Pancras Jan 2008.jpg",
            "Heathrow Terminal 5C Iwelumo-1.jpg",
            "Airbus A380-841 G-XLEB British Airways (10424102995).jpg",
            "UKpop.svg",
            "Anglospeak.svg",
            "Royal Aberdeen Children's Hospital.jpg",
            "CHANDOS3.jpg",
            "The Fabs.JPG",
            "Wembley Stadium, illuminated.jpg",
        ]
    }

    #[test]
    pub fn success_24_extract_files() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        let article = articles.get(0);
        let expected_lines = expected_lines_24();
        match article {
            None => panic!("fail to load {} article", KEYWORD),
            Some(target) => {
                let files = extract_files(target);
                assert_eq!(expected_lines.len(), files.len());
                for actual in files {
                    assert!(expected_lines.contains(&actual.as_str()));
                    assert!(
                        expected_lines.iter().any(|e| e == &actual.as_str()),
                        "{} fails",
                        actual
                    );
                    println!("[{}]", actual);
                }
            }
        }
    }

    fn expected_lines_25() -> Vec<(&'static str, &'static str)> {
        vec![
            ("略名", "イギリス"),
            ("日本語国名", "グレートブリテン及び北アイルランド連合王国"),
            ("公式国名", "{{lang|en|United Kingdom of Great Britain and Northern Ireland}}<ref>英語以外での正式国名:<br />
*{{lang|gd|An Rìoghachd Aonaichte na Breatainn Mhòr agus Eirinn mu Thuath}}（[[スコットランド・ゲール語]]）
*{{lang|cy|Teyrnas Gyfunol Prydain Fawr a Gogledd Iwerddon}}（[[ウェールズ語]]）
*{{lang|ga|Ríocht Aontaithe na Breataine Móire agus Tuaisceart na hÉireann}}（[[アイルランド語]]）
*{{lang|kw|An Rywvaneth Unys a Vreten Veur hag Iwerdhon Glédh}}（[[コーンウォール語]]）
*{{lang|sco|Unitit Kinrick o Great Breetain an Northren Ireland}}（[[スコットランド語]]）
**{{lang|sco|Claught Kängrick o Docht Brätain an Norlin Airlann}}、{{lang|sco|Unitet Kängdom o Great Brittain an Norlin Airlann}}（アルスター・スコットランド語）</ref>"),
            ("国旗画像", "Flag of the United Kingdom.svg"),
            ("国章画像", "[[ファイル:Royal Coat of Arms of the United Kingdom.svg|85px|イギリスの国章]]"),
            ("国章リンク", "（[[イギリスの国章|国章]]）"),
            ("標語", "{{lang|fr|[[Dieu et mon droit]]}}<br />（[[フランス語]]:[[Dieu et mon droit|神と我が権利]]）"),
            ("国歌", "[[女王陛下万歳|{{lang|en|God Save the Queen}}]]{{en icon}}<br />''神よ女王を護り賜え''<br />{{center|[[ファイル:United States Navy Band - God Save the Queen.ogg]]}}"),
            ("地図画像", "Europe-UK.svg"),
            ("位置画像", "United Kingdom (+overseas territories) in the World (+Antarctica claims).svg"),
            ("公用語", "[[英語]]"),
            ("首都", "[[ロンドン]]（事実上）"),
            ("最大都市", "ロンドン"),
            ("元首等肩書", "[[イギリスの君主|女王]]"),
            ("元首等氏名", "[[エリザベス2世]]"),
            ("首相等肩書", "[[イギリスの首相|首相]]"),
            ("首相等氏名", "[[ボリス・ジョンソン]]"),
            ("他元首等肩書1", "[[貴族院 (イギリス)|貴族院議長]]"),
            ("他元首等氏名1", "[[:en:Norman Fowler, Baron Fowler|ノーマン・ファウラー]]"),
            ("他元首等肩書2", "[[庶民院 (イギリス)|庶民院議長]]"),
            ("他元首等氏名2", "{{仮リンク|リンゼイ・ホイル|en|Lindsay Hoyle}}"),
            ("他元首等肩書3", "[[連合王国最高裁判所|最高裁判所長官]]"),
            ("他元首等氏名3", "[[:en:Brenda Hale, Baroness Hale of Richmond|ブレンダ・ヘイル]]"),
            ("面積順位", "76"),
            ("面積大きさ", "1 E11"),
            ("面積値", "244,820"),
            ("水面積率", "1.3%"),
            ("人口統計年", "2018"),
            ("人口順位", "22"),
            ("人口大きさ", "1 E7"),
            ("人口値", "6643万5600<ref>{{Cite web|url=https://www.ons.gov.uk/peoplepopulationandcommunity/populationandmigration/populationestimates|title=Population estimates - Office for National Statistics|accessdate=2019-06-26|date=2019-06-26}}</ref>"),
            ("人口密度値", "271"),
            ("GDP統計年元", "2012"),
            ("GDP値元", "1兆5478億<ref name=\"imf-statistics-gdp\">[http://www.imf.org/external/pubs/ft/weo/2012/02/weodata/weorept.aspx?pr.x=70&pr.y=13&sy=2010&ey=2012&scsm=1&ssd=1&sort=country&ds=.&br=1&c=112&s=NGDP%2CNGDPD%2CPPPGDP%2CPPPPC&grp=0&a=IMF>Data and Statistics>World Economic Outlook Databases>By Countrise>United Kingdom]</ref>"),
            ("GDP統計年MER", "2012"),
            ("GDP順位MER", "6"),
            ("GDP値MER", "2兆4337億<ref name=\"imf-statistics-gdp\" />"),
            ("GDP統計年", "2012"),
            ("GDP順位", "6"),
            ("GDP値", "2兆3162億<ref name=\"imf-statistics-gdp\" />"),
            ("GDP/人", "36,727<ref name=\"imf-statistics-gdp\" />"),
            ("建国形態", "建国"),
            ("確立形態1", "[[イングランド王国]]／[[スコットランド王国]]<br />（両国とも[[合同法 (1707年)|1707年合同法]]まで）"),
            ("確立年月日1", "927年／843年"),
            ("確立形態2", "[[グレートブリテン王国]]成立<br />（1707年合同法）"),
            ("確立年月日2", "1707年{{0}}5月{{0}}1日"),
            ("確立形態3", "[[グレートブリテン及びアイルランド連合王国]]成立<br />（[[合同法 (1800年)|1800年合同法]]）"),
            ("確立年月日3", "1801年{{0}}1月{{0}}1日"),
            ("確立形態4", "現在の国号「'''グレートブリテン及び北アイルランド連合王国'''」に変更"),
            ("確立年月日4", "1927年{{0}}4月12日"),
            ("通貨", "[[スターリング・ポンド|UKポンド]] (£)"),
            ("通貨コード", "GBP"),
            ("時間帯", "±0"),
            ("夏時間", "+1"),
            ("ISO 3166-1", "GB / GBR"),
            ("ccTLD", "[[.uk]] / [[.gb]]<ref>使用は.ukに比べ圧倒的少数。</ref>"),
            ("国際電話番号", "44"),
            ("注記", "<references/>"),
        ]
    }

    #[test]
    pub fn success_25_extract_basic_info() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        let article = articles.get(0);
        let expected_lines = expected_lines_25();
        match article {
            None => panic!("fail to load {} article", KEYWORD),
            Some(target) => {
                let basic_info = extract_basic_info(target, NoneCleaner {});
                assert_eq!(expected_lines.len(), basic_info.len());
                for actual in basic_info {
                    let hoge = (actual.0.as_str(), actual.1.as_str());
                    assert!(expected_lines.contains(&hoge), format!("{:?}", hoge));
                    println!("[{:?}]", actual);
                }
            }
        }
    }

    fn expected_lines_26() -> Vec<(&'static str, &'static str)> {
        vec![
            ("略名", "イギリス"),
            ("日本語国名", "グレートブリテン及び北アイルランド連合王国"),
            ("公式国名", "{{lang|en|United Kingdom of Great Britain and Northern Ireland}}<ref>英語以外での正式国名:<br />
*{{lang|gd|An Rìoghachd Aonaichte na Breatainn Mhòr agus Eirinn mu Thuath}}（[[スコットランド・ゲール語]]）
*{{lang|cy|Teyrnas Gyfunol Prydain Fawr a Gogledd Iwerddon}}（[[ウェールズ語]]）
*{{lang|ga|Ríocht Aontaithe na Breataine Móire agus Tuaisceart na hÉireann}}（[[アイルランド語]]）
*{{lang|kw|An Rywvaneth Unys a Vreten Veur hag Iwerdhon Glédh}}（[[コーンウォール語]]）
*{{lang|sco|Unitit Kinrick o Great Breetain an Northren Ireland}}（[[スコットランド語]]）
**{{lang|sco|Claught Kängrick o Docht Brätain an Norlin Airlann}}、{{lang|sco|Unitet Kängdom o Great Brittain an Norlin Airlann}}（アルスター・スコットランド語）</ref>"),
            ("国旗画像", "Flag of the United Kingdom.svg"),
            ("国章画像", "[[ファイル:Royal Coat of Arms of the United Kingdom.svg|85px|イギリスの国章]]"),
            ("国章リンク", "（[[イギリスの国章|国章]]）"),
            ("標語", "{{lang|fr|[[Dieu et mon droit]]}}<br />（[[フランス語]]:[[Dieu et mon droit|神と我が権利]]）"),
            ("国歌", "[[女王陛下万歳|{{lang|en|God Save the Queen}}]]{{en icon}}<br />神よ女王を護り賜え<br />{{center|[[ファイル:United States Navy Band - God Save the Queen.ogg]]}}"),
            ("地図画像", "Europe-UK.svg"),
            ("位置画像", "United Kingdom (+overseas territories) in the World (+Antarctica claims).svg"),
            ("公用語", "[[英語]]"),
            ("首都", "[[ロンドン]]（事実上）"),
            ("最大都市", "ロンドン"),
            ("元首等肩書", "[[イギリスの君主|女王]]"),
            ("元首等氏名", "[[エリザベス2世]]"),
            ("首相等肩書", "[[イギリスの首相|首相]]"),
            ("首相等氏名", "[[ボリス・ジョンソン]]"),
            ("他元首等肩書1", "[[貴族院 (イギリス)|貴族院議長]]"),
            ("他元首等氏名1", "[[:en:Norman Fowler, Baron Fowler|ノーマン・ファウラー]]"),
            ("他元首等肩書2", "[[庶民院 (イギリス)|庶民院議長]]"),
            ("他元首等氏名2", "{{仮リンク|リンゼイ・ホイル|en|Lindsay Hoyle}}"),
            ("他元首等肩書3", "[[連合王国最高裁判所|最高裁判所長官]]"),
            ("他元首等氏名3", "[[:en:Brenda Hale, Baroness Hale of Richmond|ブレンダ・ヘイル]]"),
            ("面積順位", "76"),
            ("面積大きさ", "1 E11"),
            ("面積値", "244,820"),
            ("水面積率", "1.3%"),
            ("人口統計年", "2018"),
            ("人口順位", "22"),
            ("人口大きさ", "1 E7"),
            ("人口値", "6643万5600<ref>{{Cite web|url=https://www.ons.gov.uk/peoplepopulationandcommunity/populationandmigration/populationestimates|title=Population estimates - Office for National Statistics|accessdate=2019-06-26|date=2019-06-26}}</ref>"),
            ("人口密度値", "271"),
            ("GDP統計年元", "2012"),
            ("GDP値元", "1兆5478億<ref name=\"imf-statistics-gdp\">[http://www.imf.org/external/pubs/ft/weo/2012/02/weodata/weorept.aspx?pr.x=70&pr.y=13&sy=2010&ey=2012&scsm=1&ssd=1&sort=country&ds=.&br=1&c=112&s=NGDP%2CNGDPD%2CPPPGDP%2CPPPPC&grp=0&a=IMF>Data and Statistics>World Economic Outlook Databases>By Countrise>United Kingdom]</ref>"),
            ("GDP統計年MER", "2012"),
            ("GDP順位MER", "6"),
            ("GDP値MER", "2兆4337億<ref name=\"imf-statistics-gdp\" />"),
            ("GDP統計年", "2012"),
            ("GDP順位", "6"),
            ("GDP値", "2兆3162億<ref name=\"imf-statistics-gdp\" />"),
            ("GDP/人", "36,727<ref name=\"imf-statistics-gdp\" />"),
            ("建国形態", "建国"),
            ("確立形態1", "[[イングランド王国]]／[[スコットランド王国]]<br />（両国とも[[合同法 (1707年)|1707年合同法]]まで）"),
            ("確立年月日1", "927年／843年"),
            ("確立形態2", "[[グレートブリテン王国]]成立<br />（1707年合同法）"),
            ("確立年月日2", "1707年{{0}}5月{{0}}1日"),
            ("確立形態3", "[[グレートブリテン及びアイルランド連合王国]]成立<br />（[[合同法 (1800年)|1800年合同法]]）"),
            ("確立年月日3", "1801年{{0}}1月{{0}}1日"),
            ("確立形態4", "現在の国号「グレートブリテン及び北アイルランド連合王国」に変更"),
            ("確立年月日4", "1927年{{0}}4月12日"),
            ("通貨", "[[スターリング・ポンド|UKポンド]] (£)"),
            ("通貨コード", "GBP"),
            ("時間帯", "±0"),
            ("夏時間", "+1"),
            ("ISO 3166-1", "GB / GBR"),
            ("ccTLD", "[[.uk]] / [[.gb]]<ref>使用は.ukに比べ圧倒的少数。</ref>"),
            ("国際電話番号", "44"),
            ("注記", "<references/>"),
        ]
    }

    #[test]
    pub fn success_26_remove_strong() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        let article = articles.get(0);
        let expected_lines = expected_lines_26();
        match article {
            None => panic!("fail to load {} article", KEYWORD),
            Some(target) => {
                let basic_info = extract_basic_info(target, StrongCleaner {});
                assert_eq!(expected_lines.len(), basic_info.len());
                for actual in basic_info {
                    let hoge = (actual.0.as_str(), actual.1.as_str());
                    assert!(expected_lines.contains(&hoge), format!("{:?}", hoge));
                    println!("[{:?}]", actual);
                }
            }
        }
    }

    fn expected_lines_27() -> Vec<(&'static str, &'static str)> {
        vec![
            ("略名", "イギリス"),
            ("日本語国名", "グレートブリテン及び北アイルランド連合王国"),
            ("公式国名", "{{lang|en|United Kingdom of Great Britain and Northern Ireland}}<ref>英語以外での正式国名:<br />
*{{lang|gd|An Rìoghachd Aonaichte na Breatainn Mhòr agus Eirinn mu Thuath}}（スコットランド・ゲール語）
*{{lang|cy|Teyrnas Gyfunol Prydain Fawr a Gogledd Iwerddon}}（ウェールズ語）
*{{lang|ga|Ríocht Aontaithe na Breataine Móire agus Tuaisceart na hÉireann}}（アイルランド語）
*{{lang|kw|An Rywvaneth Unys a Vreten Veur hag Iwerdhon Glédh}}（コーンウォール語）
*{{lang|sco|Unitit Kinrick o Great Breetain an Northren Ireland}}（スコットランド語）
**{{lang|sco|Claught Kängrick o Docht Brätain an Norlin Airlann}}、{{lang|sco|Unitet Kängdom o Great Brittain an Norlin Airlann}}（アルスター・スコットランド語）</ref>"),
            ("国旗画像", "Flag of the United Kingdom.svg"),
            ("国章画像", "ファイル:Royal Coat of Arms of the United Kingdom.svg|85px|イギリスの国章"),
            ("国章リンク", "（イギリスの国章|国章）"),
            ("標語", "{{lang|fr|Dieu et mon droit}}<br />（フランス語:Dieu et mon droit|神と我が権利）"),
            ("国歌", "女王陛下万歳|{{lang|en|God Save the Queen}}{{en icon}}<br />神よ女王を護り賜え<br />{{center|ファイル:United States Navy Band - God Save the Queen.ogg}}"),
            ("地図画像", "Europe-UK.svg"),
            ("位置画像", "United Kingdom (+overseas territories) in the World (+Antarctica claims).svg"),
            ("公用語", "英語"),
            ("首都", "ロンドン（事実上）"),
            ("最大都市", "ロンドン"),
            ("元首等肩書", "イギリスの君主|女王"),
            ("元首等氏名", "エリザベス2世"),
            ("首相等肩書", "イギリスの首相|首相"),
            ("首相等氏名", "ボリス・ジョンソン"),
            ("他元首等肩書1", "貴族院 (イギリス)|貴族院議長"),
            ("他元首等氏名1", ":en:Norman Fowler, Baron Fowler|ノーマン・ファウラー"),
            ("他元首等肩書2", "庶民院 (イギリス)|庶民院議長"),
            ("他元首等氏名2", "{{仮リンク|リンゼイ・ホイル|en|Lindsay Hoyle}}"),
            ("他元首等肩書3", "連合王国最高裁判所|最高裁判所長官"),
            ("他元首等氏名3", ":en:Brenda Hale, Baroness Hale of Richmond|ブレンダ・ヘイル"),
            ("面積順位", "76"),
            ("面積大きさ", "1 E11"),
            ("面積値", "244,820"),
            ("水面積率", "1.3%"),
            ("人口統計年", "2018"),
            ("人口順位", "22"),
            ("人口大きさ", "1 E7"),
            ("人口値", "6643万5600<ref>{{Cite web|url=https://www.ons.gov.uk/peoplepopulationandcommunity/populationandmigration/populationestimates|title=Population estimates - Office for National Statistics|accessdate=2019-06-26|date=2019-06-26}}</ref>"),
            ("人口密度値", "271"),
            ("GDP統計年元", "2012"),
            ("GDP値元", "1兆5478億<ref name=\"imf-statistics-gdp\">[http://www.imf.org/external/pubs/ft/weo/2012/02/weodata/weorept.aspx?pr.x=70&pr.y=13&sy=2010&ey=2012&scsm=1&ssd=1&sort=country&ds=.&br=1&c=112&s=NGDP%2CNGDPD%2CPPPGDP%2CPPPPC&grp=0&a=IMF>Data and Statistics>World Economic Outlook Databases>By Countrise>United Kingdom]</ref>"),
            ("GDP統計年MER", "2012"),
            ("GDP順位MER", "6"),
            ("GDP値MER", "2兆4337億<ref name=\"imf-statistics-gdp\" />"),
            ("GDP統計年", "2012"),
            ("GDP順位", "6"),
            ("GDP値", "2兆3162億<ref name=\"imf-statistics-gdp\" />"),
            ("GDP/人", "36,727<ref name=\"imf-statistics-gdp\" />"),
            ("建国形態", "建国"),
            ("確立形態1", "イングランド王国／スコットランド王国<br />（両国とも合同法 (1707年)|1707年合同法まで）"),
            ("確立年月日1", "927年／843年"),
            ("確立形態2", "グレートブリテン王国成立<br />（1707年合同法）"),
            ("確立年月日2", "1707年{{0}}5月{{0}}1日"),
            ("確立形態3", "グレートブリテン及びアイルランド連合王国成立<br />（合同法 (1800年)|1800年合同法）"),
            ("確立年月日3", "1801年{{0}}1月{{0}}1日"),
            ("確立形態4", "現在の国号「グレートブリテン及び北アイルランド連合王国」に変更"),
            ("確立年月日4", "1927年{{0}}4月12日"),
            ("通貨", "スターリング・ポンド|UKポンド (£)"),
            ("通貨コード", "GBP"),
            ("時間帯", "±0"),
            ("夏時間", "+1"),
            ("ISO 3166-1", "GB / GBR"),
            ("ccTLD", ".uk / .gb<ref>使用は.ukに比べ圧倒的少数。</ref>"),
            ("国際電話番号", "44"),
            ("注記", "<references/>"),
        ]
    }

    #[test]
    pub fn success_27_remove_internal_link() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        let article = articles.get(0);
        let expected_lines = expected_lines_27();
        match article {
            None => panic!("fail to load {} article", KEYWORD),
            Some(target) => {
                let basic_info = extract_basic_info(
                    target,
                    LinkCleaner {
                        chain: StrongCleaner {},
                    },
                );
                assert_eq!(expected_lines.len(), basic_info.len());
                for actual in basic_info {
                    let hoge = (actual.0.as_str(), actual.1.as_str());
                    assert!(expected_lines.contains(&hoge), format!("{:?}", hoge));
                    println!("[{:?}]", actual);
                }
            }
        }
    }

    #[test]
    pub fn success_29_get_image_url() {
        let articles = load_json(INPUT_PATH, KEYWORD);
        let article = articles.get(0);
        let expected_url =
            "https://upload.wikimedia.org/wikipedia/en/a/ae/Flag_of_the_United_Kingdom.svg";
        match get_country_flag_url(extract_basic_info(article.unwrap(), NoneCleaner {})) {
            None => panic!("Cannot get image url..."),
            Some(url) => {
                assert_eq!(url, expected_url);
            }
        }
    }
}
