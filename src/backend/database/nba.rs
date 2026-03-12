use uuid::Uuid;
use crate::backend::database::general::DBManager;
use crate::backend::database::general::DBPool;
use crate::backend::DbJsonError;
use crate::shared::nba::{NbaGeneral, EnumNBAGeneral, NbaGeneralPartialNoYearsNoFantasy};
use sqlx::{Row, Value};
use crate::shared::{convert_struct_to_json, DBStructSupport};

pub struct NBAGeneralBaseDB {
    pub database: DBPool
}

impl std::convert::From<DBPool> for NBAGeneralBaseDB {
    fn from(dbpool: DBPool) -> Self {
        Self { database: dbpool }
    }
}

impl DBManager for NBAGeneralBaseDB {
    type SportsType = NbaGeneral;
    async fn insert_to_db_unnest<T>(&self, sports_player_data: &Vec<Self::SportsType>) -> sqlx::Result<()>
    {
        if sports_player_data.is_empty() {
            return Ok(());
        }

        let mut uuid = Vec::new();
        let mut player_name = Vec::new();
        let mut nickname = Vec::new();
        let mut team_abbreviation = Vec::new();
        let mut age = Vec::new();
        let mut gp = Vec::new();
        let mut w = Vec::new();
        let mut l = Vec::new();
        let mut min = Vec::new();
        let mut fgm = Vec::new();
        let mut fga = Vec::new();
        let mut fg_pct = Vec::new();
        let mut fg3m = Vec::new();
        let mut fg3a = Vec::new();
        let mut fg3_pct = Vec::new();
        let mut ftm = Vec::new();
        let mut fta = Vec::new();
        let mut ft_pct = Vec::new();
        let mut oreb = Vec::new();
        let mut dreb = Vec::new();
        let mut reb = Vec::new();
        let mut ast = Vec::new();
        let mut tov = Vec::new();
        let mut stl = Vec::new();
        let mut blk = Vec::new();
        let mut blka = Vec::new();
        let mut pf = Vec::new();
        let mut pfd = Vec::new();
        let mut pts = Vec::new();
        let mut plus_minus = Vec::new();
        let mut nba_fantasy_pts = Vec::new();
        let mut dd2 = Vec::new();
        let mut td3 = Vec::new();
        let mut start_year = Vec::new();
        let mut end_year = Vec::new();

        for r in sports_player_data {
            uuid.push(Uuid::new_v4());
            player_name.push(&r.player_name);
            nickname.push(&r.nickname);
            team_abbreviation.push(&r.team);
            age.push(&r.age);
            gp.push(&r.gp);
            w.push(&r.w);
            l.push(&r.l);
            min.push(&r.min);
            fgm.push(&r.fgm);
            fga.push(&r.fga);
            fg_pct.push(&r.fg_pct);
            fg3m.push(&r.fg3m);
            fg3a.push(&r.fg3a);
            fg3_pct.push(&r.fg3_pct);
            ftm.push(&r.ftm);
            fta.push(&r.fta);
            ft_pct.push(&r.ft_pct);
            oreb.push(&r.oreb);
            dreb.push(&r.dreb);
            reb.push(&r.reb);
            ast.push(&r.ast);
            tov.push(&r.tov);
            stl.push(&r.stl);
            blk.push(&r.blk);
            blka.push(&r.blka);
            pf.push(&r.pf);
            pfd.push(&r.pfd);
            pts.push(&r.pts);
            plus_minus.push(&r.plus_minus);
            nba_fantasy_pts.push(&r.nba_fantasy_pts);
            dd2.push(&r.dd2);
            td3.push(&r.td3);
            start_year.push(&r.start_year);
            end_year.push(&r.end_year);
        }

        sqlx::query(
            r#"
            INSERT INTO nba_general (
                row_id, player_name, nickname, team_abbreviation,
                age, gp, w, l, min, fgm, fga, fg_pct,
                fg3m, fg3a, fg3_pct, ftm, fta, ft_pct,
                oreb, dreb, reb, ast, tov, stl, blk, blka,
                pf, pfd, pts, plus_minus, nba_fantasy_pts,
                dd2, td3, start_year, end_year
            )
            SELECT *
            FROM UNNEST(
                $1::uuid[],      -- row_id
                $2::text[],      -- player_name
                $3::text[],      -- nickname
                $4::text[],      -- team_abbreviation
                $5::float8[],    -- age
                $6::int2[],      -- gp
                $7::int2[],      -- w
                $8::int2[],      -- l
                $9::float8[],   -- min
                $10::float8[],   -- fgm
                $11::float8[],   -- fga
                $12::float8[],   -- fg_pct
                $13::float8[],   -- fg3m
                $14::float8[],   -- fg3a
                $15::float8[],   -- fg3_pct
                $16::float8[],   -- ftm
                $17::float8[],   -- fta
                $18::float8[],   -- ft_pct
                $19::float8[],   -- oreb
                $20::float8[],   -- dreb
                $21::float8[],   -- reb
                $22::float8[],   -- ast
                $23::float8[],   -- tov
                $24::float8[],   -- stl
                $25::float8[],   -- blk
                $26::float8[],   -- blka
                $27::float8[],   -- pf
                $28::float8[],   -- pfd
                $29::float8[],   -- pts
                $30::float8[],   -- plus_minus
                $31::float8[],   -- nba_fantasy_pts
                $32::int2[],     -- dd2
                $33::int2[],     -- td3
                $34::int2[],     -- start_year
                $35::int2[]      -- end_year
            );
        "#
        )
            // Pass references to slices for each argument
            .bind(&uuid)
            .bind(&player_name)
            .bind(&nickname)
            .bind(&team_abbreviation)
            .bind(&age)
            .bind(&gp)
            .bind(&w)
            .bind(&l)
            .bind(&min)
            .bind(&fgm)
            .bind(&fga)
            .bind(&fg_pct)
            .bind(&fg3m)
            .bind(&fg3a)
            .bind(&fg3_pct)
            .bind(&ftm)
            .bind(&fta)
            .bind(&ft_pct)
            .bind(&oreb)
            .bind(&dreb)
            .bind(&reb)
            .bind(&ast)
            .bind(&tov)
            .bind(&stl)
            .bind(&blk)
            .bind(&blka)
            .bind(&pf)
            .bind(&pfd)
            .bind(&pts)
            .bind(&plus_minus)
            .bind(&nba_fantasy_pts)
            .bind(&dd2)
            .bind(&td3)
            .bind(&start_year)
            .bind(&end_year)
            .execute(&self.database.pg_pool)
            .await?;

        Ok(())
    }

    async fn insert_to_db_querybuilder<T>(&self, sports_player_data: &Vec<serde_json::Value>) -> sqlx::Result<()> {
        todo!()
    }
    async fn query_db_year<T>(&self, table: &str, year: i32) -> Result<Vec<T>, sqlx::Error>
    where T: DBStructSupport + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {

        let query = T::select_query_year();

        sqlx::query_as::<_, T>(&query.as_str())
            .bind(year)
            .fetch_all(&self.database.pg_pool)
            .await
    }

    async fn query_db<T>(&self) -> sqlx::Result<Vec<T>, sqlx::Error> {
        todo!()
    }


    async fn update_db(&self) -> sqlx::Result<()> {
        todo!()
    }

    async fn delete_db(&self) -> sqlx::Result<()> {
        todo!()
    }

}

//keep the below since it may be faster down the road
//    async fn query_db_year(&self, table: &str, year: u32) -> sqlx::Result<Vec<Self::SportsType>> {
//         let q = format!("SELECT * FROM {} WHERE start_year = {}", table, year).to_string();
//         let players = sqlx::query_as::<_, NbaGeneral>(q.as_str()).fetch_all(&self.database.pg_pool).await?;
//         Ok(players)
//         // let query = sqlx::query(q.as_str());
//         // let rows = query.fetch_all(&self.database.pg_pool).await?;
//         //
//         // let players = rows.iter().map(|row| {
//         //     NbaGeneral {
//         //         player_name: row.get("player_name"),
//         //         nickname: row.get("nickname"),
//         //         team: row.get("team_abbreviation"),
//         //         age: row.get("age"),
//         //         gp: row.get("gp"),
//         //         w: row.get("w"),
//         //         l: row.get("l"),
//         //         min: row.get("min"),
//         //         fgm: row.get("fgm"),
//         //         fga: row.get("fga"),
//         //         fg_pct: row.get("fg_pct"),
//         //         fg3m: row.get("fg3m"),
//         //         fg3a: row.get("fg3a"),
//         //         fg3_pct: row.get("fg3_pct"),
//         //         ftm: row.get("ftm"),
//         //         fta: row.get("fta"),
//         //         ft_pct: row.get("ft_pct"),
//         //         oreb: row.get("oreb"),
//         //         dreb: row.get("dreb"),
//         //         reb: row.get("reb"),
//         //         ast: row.get("ast"),
//         //         tov: row.get("tov"),
//         //         stl: row.get("stl"),
//         //         blk: row.get("blk"),
//         //         blka: row.get("blka"),
//         //         pf: row.get("pf"),
//         //         pfd: row.get("pfd"),
//         //         pts: row.get("pts"),
//         //         plus_minus: row.get("plus_minus"),
//         //         nba_fantasy_pts: row.get("nba_fantasy_pts"),
//         //         dd2: row.get("dd2"),
//         //         td3: row.get("td3"),
//         //         start_year: row.get("start_year"),
//         //         end_year: row.get("end_year"),
//         //     }
//         // }).collect();
//
//         // Ok(players)
//     }