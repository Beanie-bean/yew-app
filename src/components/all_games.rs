use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Game {
    id: usize,
    name: AttrValue,
    year: AttrValue
}

#[derive(Properties, PartialEq)]
struct AllGamesListProps {
    all_games: Vec<Game>
}

#[component]
fn AllGamesList(AllGamesListProps { all_games }: &AllGamesListProps) -> Html {
    html! {
        for game in all_games {
            <p key={game.id}>{format!("{} {}", game.name, game.year)}</p>
        }
    }
}

#[component]
pub fn AllGames() -> Html {
    let all_games = vec![
        Game {
            id: 1,
            name: "Peli".into(),
            year: "2010".into(),
        },
    ];
    html! {
        <>
            <h1>{"All Games"}</h1>
            <AllGamesList {all_games} />
        </>
    }
}


