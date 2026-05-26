use yew::{prelude::*};

#[derive(Properties, Clone, PartialEq)]
pub struct Page {
    pub current: u32,
    pub on_click: Callback<u32>,
    pub check_next_page: Option<AttrValue>
}

#[hook]
pub fn use_pagination(page: &Page) -> (u32, Callback<u32>, Option<AttrValue>) {
    let Page { current, on_click, check_next_page } = page.clone();
    let current_page = use_state(|| current);
    
    let set_page = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| {
            if page > 0 {
                current_page.set(page);
                on_click.emit(page);
            }
        })
    };
    (*current_page, set_page, check_next_page)
}


#[component]
pub fn Pagination(page: &Page) -> Html {
    let (current_page, set_page, check_next_page) = use_pagination(&Page {
        current: page.current,
        on_click: page.on_click.clone(),
        check_next_page: page.check_next_page.clone()
    });
    
    html! {
        <nav>
            <ul class="pagination justify-content-center">
                <li class="page-item">
                    <button 
                        type="button"
                        aria-label="Go back to first page"
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(1)
                            })
                        }} disabled={current_page == 1} class="btn" style={format!("border-color: #dee2e6; border-radius: 10px 0px 0px 10px; border-right: 0px; background-color: {}; color: {}", if current_page == 1 {"#e9ecef"} else {"#ffffff"}, if current_page == 1 {"#495057"} else {"#0d6efd"})}>
                        <i class="bi bi-skip-backward"></i>
                    </button>
                </li>
                <li class="page-item">
                    <button 
                        type="button"
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(current_page.clone() - 1)
                            })
                        }} disabled={current_page == 1} class="btn" style={format!("border-color: #dee2e6; border-radius: 0px; background-color: {}; color: {}", if current_page == 1 {"#e9ecef"} else {"#ffffff"}, if current_page == 1 {"#495057"} else {"#0d6efd"})}>{"Previous"}
                    </button>
                </li>
                <li class="page-item">
                    <button 
                        type="button"
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(current_page.clone())
                            })
                        }} class="btn" style="color: #ffffff; border-radius: 0px; border-left: 0px; background-color: #0d6efd">{current_page}</button>
                </li>
                <li class="page-item">
                    <button 
                        type="button"
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(current_page.clone() + 1)
                            })
                        }} disabled={check_next_page == None} class="btn" style={format!("border-color: #dee2e6; border-radius: 0px 10px 10px 0px; border-left: 0px; background-color: {}; color: {}", if check_next_page == None {"#e9ecef"} else {"#ffffff"}, if check_next_page == None {"#495057"} else {"#0d6efd"})}>{"Next"}
                    </button>
                </li>
            </ul>
        </nav>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct MyGamesPage {
    pub current: u32,
    pub on_click: Callback<u32>,
    pub total_pages: usize
}

#[hook]
pub fn use_mygames_pagination(mygames_page: &MyGamesPage) -> (u32, Callback<u32>, usize) {
    let MyGamesPage { current, on_click, total_pages } = mygames_page.clone();
    let current_page = use_state(|| current);

    let set_page = {
        let current_page = current_page.clone();
        Callback::from(move |mygames_page: u32| {
            if mygames_page > 0 {
                current_page.set(mygames_page);
                on_click.emit(mygames_page);
            }
        })
    };
    (*current_page, set_page, total_pages)
}

#[component]
pub fn MyGamesPagination(mygames_page: &MyGamesPage) -> Html {
    let (current_page, set_page, total_pages) = use_mygames_pagination(&MyGamesPage {
        current: mygames_page.current,
        on_click: mygames_page.on_click.clone(),
        total_pages: mygames_page.total_pages.clone()
    });

html! {
        <nav>
            <ul class="pagination justify-content-center">
                <li class="page-item">
                    <button 
                        type="button"
                        aria-label="Go back to first page"
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(1)
                            })
                        }} disabled={current_page == 1} class="btn" style={format!("border-color: #dee2e6; border-radius: 10px 0px 0px 10px; border-right: 0px; background-color: {}; color: {}", if current_page == 1 {"#e9ecef"} else {"#ffffff"}, if current_page == 1 {"#495057"} else {"#0d6efd"})}>
                        <i class="bi bi-skip-backward"></i>
                    </button>
                </li>
                <li class="page-item">
                    <button 
                        type="button"
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(current_page.clone() - 1)
                            })
                        }} disabled={current_page == 1} class="btn" style={format!("border-color: #dee2e6; border-radius: 0px; background-color: {}; color: {}", if current_page == 1 {"#e9ecef"} else {"#ffffff"}, if current_page == 1 {"#495057"} else {"#0d6efd"})}>{"Previous"}
                    </button>
                </li>
                <li class="page-item">
                    <button 
                        type="button"
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(current_page.clone())
                            })
                        }} class="btn" style="color: #ffffff; border-radius: 0px; border-left: 0px; background-color: #0d6efd">{current_page}</button>
                </li>
                <li class="page-item">
                    <button 
                        type="button"
                        onclick={{
                            let set_page = set_page.clone();
                            Callback::from(move |_| {
                                set_page.emit(if current_page.clone() + 1 > total_pages.clone() as u32 {current_page.clone()} else {current_page.clone() + 1})
                            })
                        }} disabled={total_pages == current_page as usize} class="btn" style={format!("border-color: #dee2e6; border-radius: 0px 10px 10px 0px; border-left: 0px; background-color: {}; color: {}", if total_pages == current_page as usize {"#e9ecef"} else {"#ffffff"}, if total_pages == current_page as usize {"#495057"} else {"#0d6efd"})}>{"Next"}
                    </button>
                </li>
            </ul>
        </nav>
    }
}