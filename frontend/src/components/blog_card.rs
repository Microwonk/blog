use leptos::*;

#[component]
pub fn BlogCard<T>(title: T, description: T, link: T) -> impl IntoView
where
    T: Into<String> + Clone,
{
    let title_clone = title.clone();
    view! {
        <a href=link.into() class="group relative block h-64 sm:h-80 lg:h-96">
            <span class="rounded-md absolute inset-0 border-2 border-dashed border-black"></span>

            <div
                class="rounded-md relative flex h-full transform items-end border-2 border-solid border-black bg-white transition-transform group-hover:-translate-x-2 group-hover:-translate-y-2"
            >
                <div
                class="p-4 !pt-0 transition-opacity text-black group-hover:absolute group-hover:opacity-0 sm:p-6 lg:p-8"
                >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="size-10 sm:size-12"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>

                <h2 class="mt-4 text-xl text-black font-medium sm:text-2xl">{title.into()}</h2>
                </div>

                <div
                class="absolute p-4 opacity-0 transition-opacity group-hover:relative group-hover:opacity-100 sm:p-6 lg:p-8"
                >
                <h3 class="mt-4 text-xl text-black font-medium sm:text-2xl">{title_clone.into()}</h3>

                <p class="mt-4 text-sm text-black sm:text-base">
                    {description.into()}
                </p>

                <p class="mt-8 font-bold text-black">Read more</p>
                </div>
            </div>
        </a>
    }
}