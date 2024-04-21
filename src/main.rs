use std::fmt::Display;

use reqwest::Client;
use rocket::response::content::RawHtml;
use serde_json::Value;
#[macro_use]
extern crate rocket;

async fn gas_price_google() -> Result<f64, String> {
    let c = Client::default();

    let near ="https://www.google.com/s?tbm=map&suggest=p&gs_ri=maps&psi=ZeojZq3aKaCRp84PsLuKyAo.1713629801690.1&gl=us&hl=en&authuser=0&q=gas+station&ech=18&pb=!2i11!4m12!1m3!1d28229.392156029666!2d-81.34749345!3d41.14634365!2m3!1f0!2f0!3f0!3m2!1i1920!2i902!4f13.1!7i20!10b1!12m16!1m1!18b1!2m3!5m1!6e2!20e3!10b1!12b1!13b1!16b1!17m1!3e1!20m3!5e2!6b1!14b1!19m4!2m3!1i360!2i120!4i8!20m57!2m2!1i203!2i100!3m2!2i4!5b1!6m6!1m2!1i86!2i86!1m2!1i408!2i240!7m42!1m3!1e1!2b0!3e3!1m3!1e2!2b1!3e2!1m3!1e2!2b0!3e3!1m3!1e8!2b0!3e3!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e9!2b1!3e2!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e10!2b0!3e4!2b1!4b1!9b0!22m3!1sZeojZq3aKaCRp84PsLuKyAo!7e81!17sZeojZq3aKaCRp84PsLuKyAo%3A118!23m2!4b1!10b1!24m95!1m32!13m9!2b1!3b1!4b1!6i1!8b1!9b1!14b1!20b1!25b1!18m21!3b1!4b1!5b1!6b1!9b1!12b1!13b1!14b1!15b1!17b1!20b1!21b1!22b1!25b1!27m1!1b0!28b0!31b0!32b0!33m1!1b0!10m1!8e3!11m1!3e1!14m1!3b1!17b1!20m2!1e3!1e6!24b1!25b1!26b1!29b1!30m1!2b1!36b1!39m3!2m2!2i1!3i1!43b1!52b1!54m1!1b1!55b1!56m1!1b1!65m5!3m4!1m3!1m2!1i224!2i298!71b1!72m19!1m5!1b1!2b1!3b1!5b1!7b1!4b1!8m10!1m6!4m1!1e1!4m1!1e3!4m1!1e4!3sother_user_reviews!6m1!1e1!9b1!89b1!103b1!113b1!117b1!122m1!1b1!125b0!26m4!2m3!1i80!2i92!4i8!34m18!2b1!3b1!4b1!6b1!8m6!1b1!3b1!4b1!5b1!6b1!7b1!9b1!12b1!14b1!20b1!23b1!25b1!26b1!37m1!1e81!47m0!49m7!3b1!6m2!1b1!2b1!7m2!1e3!2b1!61b1!67m2!7b1!10b1!69i689";

    let price_templ = "https://www.google.com/maps/preview/place?authuser=0&hl=en&gl=us&pb=!1m24!1sCODE1%3ACODE2!3m12!1m3!1d28229.39373583169!2d-81.30938463222299!3d41.14633998037897!2m3!1f0!2f0!3f0!3m2!1i1032!2i902!4f13.1!4m2!3d41.15399!4d-81.33721!15m6!1m5!1sCODE1%3ACODE2!4s%2Fg%2F1tjypc64!5sChIJ708eMtU6MYgRz9w3PyQQMTg!6s15579647506837050151!7s108086943801208918366!6sgas%20station!12m4!2m3!1i360!2i120!4i8!13m57!2m2!1i203!2i100!3m2!2i4!5b1!6m6!1m2!1i86!2i86!1m2!1i408!2i240!7m42!1m3!1e1!2b0!3e3!1m3!1e2!2b1!3e2!1m3!1e2!2b0!3e3!1m3!1e8!2b0!3e3!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e9!2b1!3e2!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e10!2b0!3e4!2b1!4b1!9b0!14m2!1sc-ojZqaRLLjHp84PspGLoAk!7e81!15m96!1m33!4e2!13m9!2b1!3b1!4b1!6i1!8b1!9b1!14b1!20b1!25b1!18m21!3b1!4b1!5b1!6b1!9b1!12b1!13b1!14b1!15b1!17b1!20b1!21b1!22b1!25b1!27m1!1b0!28b0!30b0!32b0!33m1!1b0!10m1!8e3!11m1!3e1!14m1!3b1!17b1!20m2!1e3!1e6!24b1!25b1!26b1!29b1!30m1!2b1!36b1!39m3!2m2!2i1!3i1!43b1!52b1!54m1!1b1!55b1!56m1!1b1!65m5!3m4!1m3!1m2!1i224!2i298!71b1!72m19!1m5!1b1!2b1!3b1!5b1!7b1!4b1!8m10!1m6!4m1!1e1!4m1!1e3!4m1!1e4!3sother_user_reviews!6m1!1e1!9b1!89b1!103b1!113b1!117b1!122m1!1b1!125b0!21m0!22m2!1e81!8e4!29m0!30m5!3b1!6m1!2b1!7m1!2b1!34m2!7b1!10b1!37i689!38sCgtnYXMgc3RhdGlvbloNIgtnYXMgc3RhdGlvbpIBC2dhc19zdGF0aW9u4AEA!39sbp!40b0!41b1";

    // let t_id1="0x88313ad5321e4fef";
    // let t_id2 = "0x383110243f37dccf";
    let r = c.get(near).send().await.unwrap();
    let s = r.text_with_charset("utf-8").await.unwrap();
    let obj = serde_json::from_str::<Value>(&s[5..]).unwrap();
    let mut best_p: Option<f64> = None;
    for i in obj.as_array().unwrap()[0].as_array().unwrap()[1]
        .as_array()
        .unwrap()
        .iter()
    {
        let i = i.as_array().unwrap()[22].as_array().unwrap();
        let name = i[0].as_array().unwrap()[0].as_str().unwrap();
        let Some(id) = i[13].as_array() else {
            continue;
        };
        let id = id[0].as_array().unwrap()[0].as_str().unwrap();
        println!("{name:?} {id:?}");
        let ids: Vec<_> = id.split(':').collect();
        let [id1, id2, ..] = &ids[..] else {
            continue;
        };
        let url = price_templ.replace("CODE1", id1).replace("CODE2", id2);
        let resp = c.get(url).send().await.unwrap();
        println!("{}", resp.status());
        let s = resp.text_with_charset("utf-8").await.unwrap();

        // println!("{s}\n\n");
        // 6 88 0
        let mut ss = s.split(")]}'\n");
        // println!("{}", ss.clone().count());
        let v = format!("[{}]", ss.nth(2).unwrap());
        let obj = serde_json::from_str::<Value>(&v).unwrap();
        let price = obj.as_array().unwrap()[0].as_array().unwrap()[6]
            .as_array()
            .unwrap()[86]
            .as_array()
            .unwrap()[0]
            .as_array()
            .unwrap()[0]
            .as_array()
            .unwrap()[6]
            .as_f64();
        // println!("{}", list.len());
        // for i in list.iter().enumerate() {
        //     println!("{i:?}\n");
        // }
        // println!("{list:?}\n\n");
        println!("{price:?}");
        if best_p.is_none() || best_p > price {
            best_p = price;
        }
    }
    best_p.ok_or("No available stations found.".to_owned())
}

async fn gas_price_gasbuddy() -> Result<f64, String> {
    let c = Client::default();
    let r = c
        .get("https://www.gasbuddy.com/gasprices/ohio/columbus")
        .send()
        .await
        .unwrap();
    let s = r.text_with_charset("utf-8").await.unwrap();
    let j = serde_json::from_str::<Value>(
        s.split("window.__APOLLO_STATE__ = ")
            .nth(1)
            .unwrap()
            .split(";\n")
            .next()
            .unwrap(),
    )
    .unwrap();
    // println!("{j}");
    let o = j.as_object().unwrap();
    let mut best_gas = None;
    for i in o
        .iter()
        .enumerate()
        .filter(|i| i.1 .1["prices({\"fuel\":1})"].is_array())
        .map(|(i, (_, v))| (i, v["name"].clone(), v["prices({\"fuel\":1})"].clone()))
    {
        // println!("{i:?}");
        let prices = i.2.as_array().unwrap()[0].as_object().unwrap();
        let cash = prices
            .get("cash")
            .and_then(|c| {
                c.as_object()
                    .and_then(|c| c.get("price").and_then(|p| p.as_f64()))
            })
            .filter(|p| p != &0.0);
        let credit = prices
            .get("credit")
            .and_then(|c| {
                c.as_object()
                    .and_then(|c| c.get("price").and_then(|p| p.as_f64()))
            })
            .filter(|p| p != &0.0);
        // println!("{:?} {:?}\n", cash, credit);
        let best_gas_l = if let Some(cash) = cash {
            if let Some(credit) = credit {
                Some(cash.min(credit))
            } else {
                Some(cash)
            }
        } else {
            credit
        };
        if best_gas.is_none() || best_gas > best_gas_l {
            best_gas = best_gas_l
        }
    }
    best_gas.ok_or("No prices found".into())
}

#[derive(Debug)]
enum Source {
    Walmart,
    Target,
    Aldi,
    Meijer,
}
impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Walmart => write!(f, "Walmart"),
            Source::Target => write!(f, "Target"),
            Source::Aldi => write!(f, "Aldi's"),
            Source::Meijer => write!(f, "Meijer"),
        }
    }
}

#[derive(Debug)]
struct Item {
    name: String,
    price: f64,
    unit_price: Option<f64>,
    link: String,
    image: String,
    source: Source,
}
impl Item {
    fn to_html(&self) -> String {
        format!(
            "<h3>{}</h3><img src=\"{}\", style=\"width: 100px; height: 100px;\"><br/>Price:<b>{}</b> {} </br><a href=\"{}\">Buy item from {}</a></br></br>",
             self.name, self.image, self.price, self.unit_price.map(|u| format!("Unit price: <b>${u}</b>")).unwrap_or("".into()), self.link, self.source
        )
    }
}

#[derive(Default, Debug)]
struct ItemStack(Vec<Item>);
impl ItemStack {
    fn insert(&mut self, item: Item) {
        let ipos = self
            .0
            .iter()
            .position(|item_s| {
                if item.unit_price.is_some() && item_s.unit_price.is_some() {
                    item_s.unit_price > item.unit_price
                } else {
                    item.price < item_s.price
                }
            })
            .unwrap_or(self.0.len());
        self.0.insert(ipos, item)
    }

    fn to_html(&self) -> String {
        self.0.iter().fold(String::new(), |x, y| x + "<hr>" + &y.to_html())
    }
    fn merge(mut self, rhs: ItemStack) -> ItemStack {
        for i in rhs.0.into_iter() {
            self.insert(i);
        }
        self
    }
}

async fn get_price_of_walmart(item: &str) -> Result<ItemStack, String> {
    let c = Client::default();
    let r = c
            .get(
                r#"https://www.walmart.com/orchestra/snb/graphql/Search/cf5017327d2cf9bba91bf5c0b6c5d4a7cd4b44b06cd56e2ebddea7bd0ef51a51/search?variables={"id":"","dealsId":"","query":"PRODUCT","page":1,"prg":"desktop","catId":"","facet":"","sort":"best_match","rawFacet":"","seoPath":"","ps":40,"limit":40,"ptss":"","trsp":"","beShelfId":"","recall_set":"","module_search":"","min_price":"","max_price":"","storeSlotBooked":"","additionalQueryParams":{"hidden_facet":null,"translation":null,"isMoreOptionsTileEnabled":true,"isGenAiEnabled":true},"searchArgs":{"query":"PRODUCT","cat_id":"","prg":"desktop","facet":""},"fitmentFieldParams":{"powerSportEnabled":true,"dynamicFitmentEnabled":true,"extendedAttributesEnabled":false},"fitmentSearchParams":{"id":"","dealsId":"","query":"PRODUCT","page":1,"prg":"desktop","catId":"","facet":"","sort":"best_match","rawFacet":"","seoPath":"","ps":40,"limit":40,"ptss":"","trsp":"","beShelfId":"","recall_set":"","module_search":"","min_price":"","max_price":"","storeSlotBooked":"","additionalQueryParams":{"hidden_facet":null,"translation":null,"isMoreOptionsTileEnabled":true,"isGenAiEnabled":true},"searchArgs":{"query":"PRODUCT","cat_id":"","prg":"desktop","facet":""},"cat_id":"","_be_shelf_id":""},"enableFashionTopNav":false,"enableRelatedSearches":true,"enablePortableFacets":true,"enableFacetCount":true,"fetchMarquee":true,"fetchSkyline":true,"fetchGallery":false,"fetchSbaTop":true,"fetchDac":false,"tenant":"WM_GLASS","enableFlattenedFitment":false,"enableMultiSave":false,"enableSellerType":false,"enableAdditionalSearchDepartmentAnalytics":false,"enableFulfillmentTagsEnhacements":false,"pageType":"SearchPage"}"#
                    .replace("PRODUCT", item))
.header("x-o-segment", "oaoh")
            .header("x-o-platform-version", "us-web-1.131.0-804db680b11f6c3f50de81a358b2611151dd95d8-041813")
            .header("x-o-platform", "rweb")
            .header("X-APOLLO-OPERATION-NAME", "Search")
            .header("Cookie", "xptwg=1502316735:20DB0B5AB421BE0:5258DCF:23257895:5C5F2435:CB2A3ADE:; TS012768cf=01ac7cad5601fc3ea6b691cbc962a2b25ff7e8dade2e157635f67a1acf74f7bb1bc87f96c65b23e13402df5e8b10f950385044ebfb; TS01a90220=01ac7cad5601fc3ea6b691cbc962a2b25ff7e8dade2e157635f67a1acf74f7bb1bc87f96c65b23e13402df5e8b10f950385044ebfb; TS2a5e0c5c027=0863a7cc0fab20001f812af88298bb80ea0887f09e930c2b6807a52b32b09fec4afdef2a9723fbe808b6b897541130002023f7b1401d854557c2f42c36bbbd9ca819dd42a355f5650d8d54b34871f5bb6510777de0a9c099e8a7de801f3d72…; _pxde=7842ca903da46b82a22fb7f9f45bec1129a95f27356d6155592d73375d6dae2b:eyJ0aW1lc3RhbXAiOjE3MTM2NDcxMzQ5ODl9; _px3=a76e542bbf4ac0d4d557f95523b641e0ee1619b4d954feb0a6e7b0fd1264c786:elH6u9URRt7X6GD0qwu2d7ddZJ1gOFH+CRVXZQZWF09YUVLj48MZjOiGedoiNskIhHeyRvHK0hQE1ctHaABatg==:1000:TCeLOwpEqbm2NzBd1YAZUEUKkvKeQpT9KW/ERVJX2m7wFk7+VZYPMVmMExV7Xw0aZaH9nPbSUqWNlVqaMZawP07XKSXTBB0jRIfAwBNO/rTSdDl1qeGySP8L06Q0akjcWXc3873udDRa7qTKNYgrNTQA4UKdLF7lLmrGjF0rkaHRuMzMcqsZl7D3EqCYWAPoI74ZhuH8zSHIKhw2njnr3mAS/Qy1b9JqZP7uWbjBpWE=")
            .send().await
            .unwrap();
    let s = r.text_with_charset("utf-8").await.unwrap();
    // println!("{s}");
    // data search searchResults itemStacks [0] itemsV2 [] priceInfo currentPrice price
    let Ok(full_json) = serde_json::from_str::<Value>(&s) else {
        return Err("Walmart sux".to_owned());
    };
    let items = full_json.as_object().unwrap()["data"].as_object().unwrap()["search"]
        .as_object()
        .unwrap()["searchResult"]
        .as_object()
        .unwrap()["itemStacks"]
        .as_array()
        .unwrap()[0]
        .as_object()
        .unwrap()["itemsV2"]
        .as_array()
        .unwrap();
    // println!("{items:?}");
    let mut itemstack = ItemStack::default();
    for i in items {
        // println!("{i}");
        let Some(curprice) = i
            .as_object()
            .unwrap()
            .get("priceInfo")
            .and_then(|p| p.as_object())
        else {
            continue;
        };
        let price = dbg!(&curprice["currentPrice"])
            .as_object()
            .map(|u| u["price"].as_f64().unwrap());
        let Some(price) = price else {
            continue;
        };
        let unit_price = dbg!(&curprice["unitPrice"])
            .as_object()
            .map(|u| u["price"].as_f64().unwrap());
        let img = i.as_object().unwrap()["imageInfo"].as_object().unwrap()["thumbnailUrl"]
            .as_str()
            .unwrap()
            .to_owned();
        // println!("{price}, {unit_price}");
        let item = Item {
            name: i.as_object().unwrap()["name"].as_str().unwrap().to_owned(),
            price,
            unit_price,
            link: "www.walmart.com".to_owned()
                + i.as_object().unwrap()["canonicalUrl"].as_str().unwrap(),
            image: img,
            source: Source::Walmart,
        };
        itemstack.insert(item);
    }
    Ok(itemstack)
}

async fn get_price_of_target(item: &str) -> Result<ItemStack, String> {
    let c = Client::default();
    let mut cookie = String::new();
    let resp = c.get("https://www.target.com/").send().await.unwrap();
    // println!("{:?}",resp.headers());
    cookie = resp
        .headers()
        .iter()
        .filter(|(k, _)| k.as_str() == "set-cookie")
        .fold(cookie, |s, t| s + t.1.to_str().unwrap());
    let visit_id = resp
        .headers()
        .iter()
        .filter_map(|(_, v)| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .find(|v| v.contains("visitorId"))
        .map(|i| i.split('=').nth(1).unwrap());
    let Some(visit_id) = visit_id else {
        return Err("No target".to_owned());
    };
    // println!("{cookie} {visit_id}");
    let resp = c.get("https://redsky.target.com/redsky_aggregations/v1/web/plp_search_v2?key=9f36aeafbe60771e321a7cc95a78140772ab3e96&channel=WEB&count=24&default_purchasability_filter=true&include_dmc_dmr=true&include_sponsored=true&keyword=PRODUCT&new_search=true&offset=0&page=/s/PRODUCT&platform=desktop&pricing_store_id=988&store_ids=988,793,2157,986,1841&visitor_id=VISIT&zip=44242".replace("PRODUCT", item).replace("VISIT", visit_id)).header("visitorId", visit_id)
            .header("Cookie",cookie )
            .send().await.unwrap();
    let s = resp.text_with_charset("utf-8").await.unwrap();
    // println!("{s}");

    // data search products [] item product_description title
    // data search products [] price current_retail/formatted_unit_price
    let parse_json = serde_json::from_str::<Value>(&s).unwrap();
    let Some(prods) = dbg!(parse_json.as_object().unwrap())["data"]
        .as_object()
        .unwrap()
        .get("search")
    else {
        return Err("Not found".into());
    };
    let prods = prods.as_object().unwrap()["products"].as_array().unwrap();
    // println!("{}", prods.len());
    let mut itemstack = ItemStack::default();
    for i in prods {
        // println!("{i:?}");
        let name: String = i.as_object().unwrap()["item"].as_object().unwrap()
            ["product_description"]
            .as_object()
            .unwrap()["title"]
            .as_str()
            .unwrap()
            .to_owned();
        let price = i.as_object().unwrap()["price"].as_object().unwrap()["current_retail"]
            .as_f64()
            .unwrap();
        let unit_price = i.as_object().unwrap()["price"]
            .as_object()
            .unwrap()
            .get("formatted_unit_price")
            .map(|f| f.as_str().unwrap()[1..].to_owned().parse().unwrap());
        let link = i.as_object().unwrap()["item"].as_object().unwrap()["enrichment"]
            .as_object()
            .unwrap()["buy_url"]
            .as_str()
            .unwrap()
            .to_owned();
        let img = i.as_object().unwrap()["item"].as_object().unwrap()["enrichment"]
            .as_object()
            .unwrap()["images"]
            .as_object()
            .unwrap()["primary_image_url"]
            .as_str()
            .unwrap()
            .to_owned();
        // println!("{name} {price}")
        let item = Item {
            name,
            price,
            unit_price: (unit_price),
            link,
            image: img,
            source: Source::Target,
        };
        itemstack.insert(item);
    }
    Ok(itemstack)
}

async fn get_price_of_aldis(item: &str) -> Result<ItemStack, String> {
    let c = Client::new();
    let resp = c.get("https://api.aldi.us/v1/catalog-search-product-offers?currency=USD&serviceType=pickup&q=PRODUCT&page%5Blimit%5D=24&page%5Boffset%5D=0&sort=relevance&merchantReference=463-083".replace("PRODUCT", item)).send().await.unwrap();
    let s = resp.text_with_charset("utf-8").await.unwrap();
    let json = serde_json::from_str::<Value>(&s).unwrap();

    // data [0] attributes catalogSearchProductOfferResults [] prices netAmount/100.
    // [] name
    // [] images [0] externalUrlLarge
    // [] netContent str->f64
    let mut itemstack = ItemStack::default();
    for item in json.as_object().unwrap()["data"].as_array().unwrap()[0]
        .as_object()
        .unwrap()["attributes"]
        .as_object()
        .unwrap()["catalogSearchProductOfferResults"]
        .as_array()
        .unwrap()
    {
        // println!("{item}");
        let price = item.as_object().unwrap()["prices"].as_array().unwrap()[0]
            .as_object()
            .unwrap()["netAmount"]
            .as_f64()
            .unwrap()
            / 100.;
        let name = item.as_object().unwrap()["name"]
            .as_str()
            .unwrap()
            .to_owned();
        let img = item.as_object().unwrap()["images"]
            .as_array()
            .unwrap()
            .get(0)
            .map(|i| {
                i.as_object().unwrap()["externalUrlLarge"]
                    .as_str()
                    .unwrap()
                    .replace("{width}", "100")
            });
        let link = "https://new.aldi.us/product/".to_owned()
            + item.as_object().unwrap()["urlSlugText"].as_str().unwrap();
        itemstack.insert(Item {
            name,
            price,
            unit_price: None,
            link,
            image: img.unwrap_or("".to_owned()),
            source: Source::Aldi,
        })
    }
    Ok(itemstack)
}

// fn main() {
//     // println!("{:?}", GasBuddy.gas_price());
//     rocket
//     println!("{:?}", Walmart.get_price_of("bacon"));
// }

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(
        r##"<html><head><script src="https://unpkg.com/htmx.org@1.9.12" integrity="sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2" crossorigin="anonymous"></script><style>
#load{
        opacity:0;
        transition: opacity 500ms ease-in;
    }
    .htmx-request #load{
        opacity:1
    }
    .htmx-request#load{
        opacity:1
    }
        </style><title>PriceSmart</title></head><body><form hx-target="#results" hx-post="/search" hx-indicator="#load"><input name="prod" id="prod"><button type="submit" hx_>Submit</button></form><p id="load" >Searching...</p><p id="results"></p></body></html>"##,
    )
}

async fn get_price_of_meijer(item: &str) -> Result<ItemStack, String> {
    let c = Client::new();
    let resp = c.get("https://ac.cnstrc.com/search/PRODUCT?key=key_GdYuTcnduTUtsZd6&s=1&us=web&page=1&num_results_per_page=52&filters%5BavailableInStores%5D=317&sort_by=relevance&sort_order=descending&fmt_options%5Bgroups_max_depth%5D=3&fmt_options%5Bgroups_start%5D=current&_dt=1713665760722".replace("PRODUCT", item)).send().await.unwrap();
    let s = resp.text_with_charset("utf-8").await.unwrap();
    let json = serde_json::from_str::<Value>(&s).unwrap();
    let items = json.as_object().unwrap()["response"].as_object().unwrap()["results"]
        .as_array()
        .unwrap();
    let mut itemstack = ItemStack::default();
    for i in items {
        let data = i.as_object().unwrap()["data"].as_object().unwrap();

        let name = i.as_object().unwrap()["value"].as_str().unwrap().to_owned();
        let price = data["price"].as_f64().unwrap();
        let link = data["url"].as_str().unwrap().to_owned();
        let image = data["image_url"].as_str().unwrap().to_owned();

        itemstack.insert(Item {
            name,
            price,
            unit_price: None,
            link,
            image,
            source: Source::Meijer,
        })
    }
    Ok(itemstack)
}

#[post("/search", data = "<prodd>")]
async fn search(prodd: String) -> Result<RawHtml<String>, String> {
    let (_, prod) = prodd.split_once('=').ok_or("Invalid form".to_owned())?;
    let mut items = ItemStack::default();
    let mut prefix = String::new();
    let target_res = get_price_of_target(prod).await;
    if let Ok(target) = target_res {
        items = items.merge(target);
    } else {
        prefix += "Target didnt load</br>";
    }
    let walmart_res = get_price_of_walmart(prod).await;
    if let Ok(walmart) = walmart_res {
        items = items.merge(walmart);
    } else {
        prefix += "Walmart didnt load</br>";
    }
    let aldis_res = get_price_of_aldis(prod).await;
    if let Ok(aldis) = aldis_res {
        items = items.merge(aldis);
    } else {
        prefix += "Aldi didnt load</br>";
    }
    let meijer_res = get_price_of_meijer(prod).await;
    if let Ok(meijer) = meijer_res {
        items = items.merge(meijer);
    } else {
        prefix += "Meijer didnt load</br>";
    }
    Ok(RawHtml(prefix + &items.to_html()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}
