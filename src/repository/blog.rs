use std::vec;

use uuid::Uuid;

use crate::domain::blog::Blog;
use crate::domain::user::User;

pub trait BlogRepository {
    fn get_blog(&self, id: Uuid) -> Blog;
    // fn create_blog(&self, blog: Blog);
    // fn delete_blog(&self, id: Uuid);
}

pub struct MockBlogRepository;
impl BlogRepository for MockBlogRepository {
    fn get_blog(&self, id: Uuid) -> Blog {
        Blog {
            id,
            title: "test title".to_string(),
            content: "私はその人を常に先生と呼んでいた。だからここでもただ先生と書くだけで本名は打ち明けない。これは世間を憚かる遠慮というよりも、その方が私にとって自然だからである。私はその人の記憶を呼び起すごとに、すぐ「先生」といいたくなる。筆を執っても心持は同じ事である。よそよそしい頭文字などはとても使う気にならない。私が先生と知り合いになったのは鎌倉である。その時私はまだ若々しい書生であった。暑中休暇を利用して海水浴に行った友達からぜひ来いという端書を受け取ったので、私は多少の金を工面して、出掛ける事にした。私は金の工面に二、三日を費やした。ところが私が鎌倉に着いて三日と経たないうちに、私を呼び寄せた友達は、急に国元から帰れという電報を受け取った。電報には母が病気だからと断ってあったけれども友達はそれを信じなかった。友達はかねてから国元にいる親たちに勧まない結婚を強いられていた。彼は現代の習慣からいうと結婚するにはあまり年が若過ぎた。それに肝心の当人が気に入らなかった。それで夏休みに当然帰るべきところを、わざと避けて東京の近くで遊んでいたのである。彼は電報を私に見せてどうしようと相談をした。私にはどう".to_string(),
            authors: vec![
                User {
                    id: uuid::Uuid::new_v4(),
                    name: "itt".to_string(),
                },
                User {
                    id: uuid::Uuid::new_v4(),
                    name: "iitt".to_string(),
                },
            ],
            tags: vec![],
        }
    }
}
