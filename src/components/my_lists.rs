use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct List {
    id: usize,
    name: AttrValue,
    desc: AttrValue
}

#[derive(Properties, PartialEq)]
struct MyListsListProps {
    my_lists: Vec<List>
}

#[component]
fn MyListsList(MyListsListProps { my_lists }: &MyListsListProps) -> Html {
    html! {
        for list in my_lists {
            <p key={list.id}>{format!("{} {}", list.name, list.desc)}</p>
        }
    }
}

#[component]
pub fn MyLists() -> Html {
    let my_lists = vec![
        List {
            id: 1,
            name: "Lista".into(),
            desc: "kuvaus".into(),
        },
    ];
    html! {
        <>
            <h1>{"My Lists"}</h1>
            <MyListsList {my_lists} />
        </>
    }
}


