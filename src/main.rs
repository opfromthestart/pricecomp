use std::io::Read;

use reqwest::blocking::Client;
use serde_json::Value;

trait GasPrice {
    fn gas_price(&self) -> Result<f64, String>;
}

struct GoogleMaps;
impl GasPrice for GoogleMaps {
    fn gas_price(&self) -> Result<f64, String> {
        let c = Client::default();

        let near ="https://www.google.com/s?tbm=map&suggest=p&gs_ri=maps&psi=ZeojZq3aKaCRp84PsLuKyAo.1713629801690.1&gl=us&hl=en&authuser=0&q=gas+station&ech=18&pb=!2i11!4m12!1m3!1d28229.392156029666!2d-81.34749345!3d41.14634365!2m3!1f0!2f0!3f0!3m2!1i1920!2i902!4f13.1!7i20!10b1!12m16!1m1!18b1!2m3!5m1!6e2!20e3!10b1!12b1!13b1!16b1!17m1!3e1!20m3!5e2!6b1!14b1!19m4!2m3!1i360!2i120!4i8!20m57!2m2!1i203!2i100!3m2!2i4!5b1!6m6!1m2!1i86!2i86!1m2!1i408!2i240!7m42!1m3!1e1!2b0!3e3!1m3!1e2!2b1!3e2!1m3!1e2!2b0!3e3!1m3!1e8!2b0!3e3!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e9!2b1!3e2!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e10!2b0!3e4!2b1!4b1!9b0!22m3!1sZeojZq3aKaCRp84PsLuKyAo!7e81!17sZeojZq3aKaCRp84PsLuKyAo%3A118!23m2!4b1!10b1!24m95!1m32!13m9!2b1!3b1!4b1!6i1!8b1!9b1!14b1!20b1!25b1!18m21!3b1!4b1!5b1!6b1!9b1!12b1!13b1!14b1!15b1!17b1!20b1!21b1!22b1!25b1!27m1!1b0!28b0!31b0!32b0!33m1!1b0!10m1!8e3!11m1!3e1!14m1!3b1!17b1!20m2!1e3!1e6!24b1!25b1!26b1!29b1!30m1!2b1!36b1!39m3!2m2!2i1!3i1!43b1!52b1!54m1!1b1!55b1!56m1!1b1!65m5!3m4!1m3!1m2!1i224!2i298!71b1!72m19!1m5!1b1!2b1!3b1!5b1!7b1!4b1!8m10!1m6!4m1!1e1!4m1!1e3!4m1!1e4!3sother_user_reviews!6m1!1e1!9b1!89b1!103b1!113b1!117b1!122m1!1b1!125b0!26m4!2m3!1i80!2i92!4i8!34m18!2b1!3b1!4b1!6b1!8m6!1b1!3b1!4b1!5b1!6b1!7b1!9b1!12b1!14b1!20b1!23b1!25b1!26b1!37m1!1e81!47m0!49m7!3b1!6m2!1b1!2b1!7m2!1e3!2b1!61b1!67m2!7b1!10b1!69i689";

        let price_templ = "https://www.google.com/maps/preview/place?authuser=0&hl=en&gl=us&pb=!1m24!1sCODE1%3ACODE2!3m12!1m3!1d28229.39373583169!2d-81.30938463222299!3d41.14633998037897!2m3!1f0!2f0!3f0!3m2!1i1032!2i902!4f13.1!4m2!3d41.15399!4d-81.33721!15m6!1m5!1sCODE1%3ACODE2!4s%2Fg%2F1tjypc64!5sChIJ708eMtU6MYgRz9w3PyQQMTg!6s15579647506837050151!7s108086943801208918366!6sgas%20station!12m4!2m3!1i360!2i120!4i8!13m57!2m2!1i203!2i100!3m2!2i4!5b1!6m6!1m2!1i86!2i86!1m2!1i408!2i240!7m42!1m3!1e1!2b0!3e3!1m3!1e2!2b1!3e2!1m3!1e2!2b0!3e3!1m3!1e8!2b0!3e3!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e9!2b1!3e2!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e10!2b0!3e4!2b1!4b1!9b0!14m2!1sc-ojZqaRLLjHp84PspGLoAk!7e81!15m96!1m33!4e2!13m9!2b1!3b1!4b1!6i1!8b1!9b1!14b1!20b1!25b1!18m21!3b1!4b1!5b1!6b1!9b1!12b1!13b1!14b1!15b1!17b1!20b1!21b1!22b1!25b1!27m1!1b0!28b0!30b0!32b0!33m1!1b0!10m1!8e3!11m1!3e1!14m1!3b1!17b1!20m2!1e3!1e6!24b1!25b1!26b1!29b1!30m1!2b1!36b1!39m3!2m2!2i1!3i1!43b1!52b1!54m1!1b1!55b1!56m1!1b1!65m5!3m4!1m3!1m2!1i224!2i298!71b1!72m19!1m5!1b1!2b1!3b1!5b1!7b1!4b1!8m10!1m6!4m1!1e1!4m1!1e3!4m1!1e4!3sother_user_reviews!6m1!1e1!9b1!89b1!103b1!113b1!117b1!122m1!1b1!125b0!21m0!22m2!1e81!8e4!29m0!30m5!3b1!6m1!2b1!7m1!2b1!34m2!7b1!10b1!37i689!38sCgtnYXMgc3RhdGlvbloNIgtnYXMgc3RhdGlvbpIBC2dhc19zdGF0aW9u4AEA!39sbp!40b0!41b1";

        // let t_id1="0x88313ad5321e4fef";
        // let t_id2 = "0x383110243f37dccf";
        let mut r = c.get(near).send().unwrap();
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
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
            let mut resp = c.get(url).send().unwrap();
            println!("{}", resp.status());
            resp.read_to_string(&mut s).unwrap();

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
}

struct GasBuddy;

impl GasPrice for GasBuddy {
    fn gas_price(&self) -> Result<f64, String> {
        let c = Client::default();
        let mut r = c
            .get("https://www.gasbuddy.com/gasprices/ohio/columbus")
            .send()
            .unwrap();
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
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
}
#[derive(Debug)]
struct Item {
    name: String,
    price: f64,
    unit_price: Option<f64>,
    link: String,
}

trait Store {
    fn get_price_of(&self, item: &str) -> Result<Item, String>;
}

struct Walmart;
impl Store for Walmart {
    fn get_price_of(&self, item: &str) -> Result<Item, String> {
        let c = Client::default();
        let mut r = c
            .get(
                r#"https://www.walmart.com/orchestra/snb/graphql/Search/cf5017327d2cf9bba91bf5c0b6c5d4a7cd4b44b06cd56e2ebddea7bd0ef51a51/search?variables={"id":"","dealsId":"","query":"bacon","page":1,"prg":"desktop","catId":"","facet":"","sort":"best_match","rawFacet":"","seoPath":"","ps":40,"limit":40,"ptss":"","trsp":"","beShelfId":"","recall_set":"","module_search":"","min_price":"","max_price":"","storeSlotBooked":"","additionalQueryParams":{"hidden_facet":null,"translation":null,"isMoreOptionsTileEnabled":true,"isGenAiEnabled":true},"searchArgs":{"query":"PRODUCT","cat_id":"","prg":"desktop","facet":""},"fitmentFieldParams":{"powerSportEnabled":true,"dynamicFitmentEnabled":true,"extendedAttributesEnabled":false},"fitmentSearchParams":{"id":"","dealsId":"","query":"PRODUCT","page":1,"prg":"desktop","catId":"","facet":"","sort":"best_match","rawFacet":"","seoPath":"","ps":40,"limit":40,"ptss":"","trsp":"","beShelfId":"","recall_set":"","module_search":"","min_price":"","max_price":"","storeSlotBooked":"","additionalQueryParams":{"hidden_facet":null,"translation":null,"isMoreOptionsTileEnabled":true,"isGenAiEnabled":true},"searchArgs":{"query":"PRODUCT","cat_id":"","prg":"desktop","facet":""},"cat_id":"","_be_shelf_id":""},"enableFashionTopNav":false,"enableRelatedSearches":true,"enablePortableFacets":true,"enableFacetCount":true,"fetchMarquee":true,"fetchSkyline":true,"fetchGallery":false,"fetchSbaTop":true,"fetchDac":false,"tenant":"WM_GLASS","enableFlattenedFitment":false,"enableMultiSave":false,"enableSellerType":false,"enableAdditionalSearchDepartmentAnalytics":false,"enableFulfillmentTagsEnhacements":false,"pageType":"SearchPage"}"#
                    .replace("PRODUCT", item))
.header("x-o-segment", "oaoh")
            .header("x-o-platform-version", "us-web-1.131.0-804db680b11f6c3f50de81a358b2611151dd95d8-041813")
            .header("x-o-platform", "rweb")
            .header("X-APOLLO-OPERATION-NAME", "Search")
            .header("Cookie", "xptwg=1502316735:20DB0B5AB421BE0:5258DCF:23257895:5C5F2435:CB2A3ADE:; TS012768cf=01ac7cad5601fc3ea6b691cbc962a2b25ff7e8dade2e157635f67a1acf74f7bb1bc87f96c65b23e13402df5e8b10f950385044ebfb; TS01a90220=01ac7cad5601fc3ea6b691cbc962a2b25ff7e8dade2e157635f67a1acf74f7bb1bc87f96c65b23e13402df5e8b10f950385044ebfb; TS2a5e0c5c027=0863a7cc0fab20001f812af88298bb80ea0887f09e930c2b6807a52b32b09fec4afdef2a9723fbe808b6b897541130002023f7b1401d854557c2f42c36bbbd9ca819dd42a355f5650d8d54b34871f5bb6510777de0a9c099e8a7de801f3d72…; _pxde=7842ca903da46b82a22fb7f9f45bec1129a95f27356d6155592d73375d6dae2b:eyJ0aW1lc3RhbXAiOjE3MTM2NDcxMzQ5ODl9; _px3=a76e542bbf4ac0d4d557f95523b641e0ee1619b4d954feb0a6e7b0fd1264c786:elH6u9URRt7X6GD0qwu2d7ddZJ1gOFH+CRVXZQZWF09YUVLj48MZjOiGedoiNskIhHeyRvHK0hQE1ctHaABatg==:1000:TCeLOwpEqbm2NzBd1YAZUEUKkvKeQpT9KW/ERVJX2m7wFk7+VZYPMVmMExV7Xw0aZaH9nPbSUqWNlVqaMZawP07XKSXTBB0jRIfAwBNO/rTSdDl1qeGySP8L06Q0akjcWXc3873udDRa7qTKNYgrNTQA4UKdLF7lLmrGjF0rkaHRuMzMcqsZl7D3EqCYWAPoI74ZhuH8zSHIKhw2njnr3mAS/Qy1b9JqZP7uWbjBpWE=")
            .send()
            .unwrap();
        let mut s = String::new();
        r.read_to_string(&mut s).unwrap();
        println!("{s}");
        // data search searchResults itemStacks [0] itemsV2 [] priceInfo currentPrice price
        let full_json = serde_json::from_str::<Value>(&s).unwrap();
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
        let mut best_item = None;
        for i in items {
            println!("{i}");
            let Some(curprice) = i.as_object().unwrap().get("priceInfo").and_then(|p| p.as_object()) else {
                continue;
            };
            let price = curprice["currentPrice"].as_object().unwrap()["price"]
                .as_f64()
                .unwrap();
            let unit_price = curprice["unitPrice"].as_object().unwrap()["price"]
                .as_f64()
                .unwrap();
            println!("{price}, {unit_price}");
            let item = Item {
                        name: i.as_object().unwrap()["name"].as_str().unwrap().to_owned(),
                        price,
                        unit_price: Some(unit_price),
                        link: "www.walmart.com".to_owned()+i.as_object().unwrap()["canonicalUrl"].as_str().unwrap() ,
                    };
            if let Some(Item { unit_price: Some(u), .. }) = best_item {
                if unit_price < u {
                    best_item = Some(item)
                }
            } else {
                best_item = Some(item)
            }
        }
        best_item.ok_or("Item not found".to_owned())

    }
}

fn main() {
    // println!("{:?}", GasBuddy.gas_price());
    println!("{:?}", Walmart.get_price_of("eggs"));
}
