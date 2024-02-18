"use client";
import {useEffect, useState} from "react";
import {login} from "@/lib/login"

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [sessionid, setSessionid] = useState("");
  const [key, setKey] = useState("");
  const [loading, setLoading] = useState("");

  function setCookie(cName, cValue, expDays) {
    let date = new Date();
    date.setTime(date.getTime() + (expDays * 24 * 60 * 60 * 1000));
    const expires = "expires=" + date.toUTCString();
    document.cookie = cName + "=" + cValue + "; " + expires + "; path=/";
  }

  function getCookie(cName) {
    const name = cName + "=";
    const cDecoded = decodeURIComponent(document.cookie); //to be careful
    const cArr = cDecoded .split('; ');
    let res;
    cArr.forEach(val => {
      if (val.indexOf(name) === 0) res = val.substring(name.length);
    })
    return res;
  }

  // useEffect(() => {
  //
  // }, []);

  const handleSubmit = (e) => {
    e.preventDefault();
    // let key_val;
    // key_val = login(email,password);
    // setKey(key_val);
    login(email, password).then((r) => {
      setSessionid(r);
      console.log(sessionid);
      if (sessionid !== undefined && sessionid.length > 30 && sessionid !== "<html><body><h1>429 Too Many Requests</h1>") {
        setCookie('session_id', sessionid, 30);
        window.location.replace("/dashboard");
      } else {
        setCookie('session_id', '', 0);

        setEmail('');
        setPassword('');
      }
    })


  }

  return (
      <>
        <div className="flex min-h-full flex-1 items-center justify-center px-4 py-12 sm:px-6 lg:px-8">
          <div className="w-full max-w-sm space-y-10">
            <div>
              <img
                  className="mx-auto h-10 w-auto"
                  src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                  alt="AquariQuality"
              />
              <h2 className="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                Sign in to your account
              </h2>
            </div>
            <form className="space-y-6" action="#" method="POST">
              <div className="relative -space-y-px rounded-md shadow-sm">
                <div className="pointer-events-none absolute inset-0 z-10 rounded-md ring-1 ring-inset ring-gray-300"/>
                <label htmlFor="email-address" className="sr-only">
                  Email address
                </label>
                <input
                    type="email"
                    autoComplete="email"
                    onChange={(e) => setEmail(e.target.value)}
                    value={email}
                    required
                    className="relative block w-full rounded-t-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    placeholder="Email address"
                />
                <label htmlFor="password" className="sr-only">
                  Password
                </label>
                <input
                    type="password"
                    autoComplete="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    required
                    className="relative block w-full rounded-b-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    placeholder="Password"
                />
              </div>

              <div className="flex items-center justify-between">
                <div className="flex items-center">
                  <input
                      id="remember-me"
                      name="remember-me"
                      type="checkbox"
                      className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                  />
                  <label htmlFor="remember-me" className="ml-3 block text-sm leading-6 text-gray-900">
                    Remember me
                  </label>
                </div>

                <div className="text-sm leading-6">
                  <a href="#" className="font-semibold text-indigo-600 hover:text-indigo-500">
                    Forgot password?
                  </a>
                </div>
              </div>

              <div>
                <button
                    type="submit"
                    onClick={(e) => handleSubmit(e)}
                    className="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >
                  Sign in
                </button>
              </div>
            </form>

            <p className="text-center text-sm leading-6 text-gray-500">
              Not a member?{' '}
              <a href="#" className="font-semibold text-indigo-600 hover:text-indigo-500">
                Register now
              </a>
            </p>
          </div>
        </div>
      </>
  )
}
