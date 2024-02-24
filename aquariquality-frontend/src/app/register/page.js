"use client";
import {useEffect, useState} from "react";
import {login} from "@/lib/login"
import {setCookie} from "@/lib/cookies";

export default function Register() {
  const [ip, setIp] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [authtoken, setAuthtoken] = useState("");
  const [auth, setAuth] = useState(false);
  const [failure, setFailure] = useState(false);
  const [loading, setLoading] = useState(false);
  const [contactip, setContactip] = useState("");
  const [analyticSent, setanalyticSent] = useState(false);
  const [sessionid, setSessionid] = useState("");
  const [key, setKey] = useState("");
  const [refresh, setRefresh] = useState("");


  // useEffect( () => {
  //   getIp();
  //   if (!analyticSent && contactip.length > 0) {
  //     console.log(contactip);
  //     let cookie_set = getCookie("session_id");
  //     const res = sendAnalytic(contactip, navigator.userAgent, window.location.href, cookie_set);
  //     setanalyticSent(true);
  //   }
  //   console.log(ip)
  // }, [ip, contactip, analyticSent])

  const handleSubmit = (e) => {
    e.preventDefault();

    const json = JSON.stringify({"email": email, "password": password});

    register(email,password)
        .then((data) => {
          setLoading(false);
          setEmail('');
          setPassword('');
        })
        .catch((error) => {
          console.log(error);
          setAuth("");
          setLoading(false);
        });
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
