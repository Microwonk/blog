use leptos::*;
use leptos_meta::*;
use leptos_router::{use_navigate, NavigateOptions};
use regex::Regex;

use crate::{types::Profile, util::Api};

#[component]
pub fn LoginPage(
    set_logged_in: WriteSignal<bool>,
    set_user: WriteSignal<Option<Profile>>,
) -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (email_error, set_email_error) = create_signal(None::<String>);
    let (password_error, set_password_error) = create_signal(None::<String>);

    view! {
        <Title text="Login"/>
        <div class="mx-auto max-w-screen-xl px-4 py-16 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-lg">
                <h1 class="text-center text-2xl font-bold text-black sm:text-3xl">Sign in</h1>

                <p class="mx-auto mt-4 max-w-md text-center text-gray-500">
                    Sign in to post comments!
                </p>

                <div class="mb-0 mt-6 space-y-4 rounded-lg p-4 shadow-lg sm:p-6 lg:p-8">

                    <div>
                        <label for="email" class="sr-only">Email</label>

                        <div class="relative">
                            <input
                                prop:type="email"
                                class="rounded-lg border-gray-200 p-4 pe-12 text-sm shadow-sm"
                                style="width: 85%;"
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_email(value);
                                    set_email_error(None); // Reset error on input
                                }
                                prop:value=email
                                placeholder="Enter email"
                            />

                            <span class="absolute inset-y-0 end-0 grid place-content-center px-4">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="size-4 text-gray-400"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"
                                    />
                                </svg>
                            </span>
                        </div>
                        // Display email error message
                        { move || email_error.get().map(|error| view! {
                            <p class="text-red-500 text-sm mt-2">{error}</p>
                        }) }
                    </div>

                    <div>
                        <label for="password" class="sr-only">Password</label>

                        <div class="relative">
                            <input
                                prop:type="password"
                                class="rounded-lg border-gray-200 p-4 pe-12 text-sm shadow-sm"
                                style="width: 85%;"
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_password(value);
                                    set_password_error(None); // Reset error on input
                                }
                                prop:value=password
                                placeholder="Enter password"
                            />

                            <span class="absolute inset-y-0 end-0 grid place-content-center px-4">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="size-4 text-gray-400"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                    />
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                                    />
                                </svg>
                            </span>
                        </div>
                        // Display password error message
                        { move || password_error.get().map(|error| view! {
                            <p class="text-red-500 text-sm mt-2">{error}</p>
                        }) }
                    </div>

                    <button
                        class="block w-full rounded-lg bg-black px-5 py-3 text-sm font-medium text-white"
                        on:click=move |_| {
                            let email_value = email.get();
                            let password_value = password.get();

                            let mut valid = true;

                            if !Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
                                    .unwrap()
                                    .is_match(email_value.as_str())
                            {
                                set_email_error(Some("Please enter a valid email address.".to_string()));
                                valid = false;
                            }

                            if password_value.len() < 8 {
                                set_password_error(Some("Password must be at least 8 characters long.".to_string()));
                                valid = false;
                            }

                            spawn_local(async move {
                                if valid {
                                    #[allow(clippy::single_match)]
                                    match Api::login(email_value, password_value).await {
                                        Ok(_) => {
                                            set_logged_in(true);
                                            set_user(Api::get_profile().await.ok());
                                            let navigate = use_navigate();
                                            navigate("/", NavigateOptions::default());
                                        },
                                        Err(_) => {
                                            set_email_error(None);
                                            set_password_error(Some("Wrong password or E-Mail.".to_string()));
                                        }
                                    }
                                }
                            });

                        }
                    >
                        Sign in
                    </button>

                    <p class="text-center text-sm text-gray-500">
                        No account?
                        <a class="underline text-black" href="/register">Sign up</a>
                    </p>
                </div>
            </div>
        </div>
    }
}
