"use client";
import Image from "next/image";
import {useEffect, useState} from "react";
import {getCookie} from "@/lib/cookies";
import {verify_login} from "@/lib/login";

export default function Dashboard() {
  const [authenticated, setAuthenticated] = useState("false");
  const [cookie, setCookie] = useState("d");


    useEffect(() => {
        let cookie = getCookie('session_id');
        verify_login(cookie)
            .then((r) => r.toString())
            .then((data) => {
            console.log(data)
            let res;
            res = data;
            if (res === "true") {
                setAuthenticated("true")
            }
        })

    }, []);


  return (
      <>
        {authenticated ? (
            <div>AUTHENTICATED!!</div>
        ): (
            <div>...not authenticated...<br/>{authenticated.toString()}<br/>{cookie}</div>
        )}

      </>

  );
}
