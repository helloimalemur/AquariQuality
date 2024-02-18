"use client";
import Image from "next/image";
import {useEffect, useState} from "react";
import {getCookie} from "@/lib/cookies";
import {verify_login} from "@/lib/login";

export default function Dashboard() {
  const [authenticated, setAuthenticated] = useState("false");
  const [cookie, setCookie] = useState("");
  const [loading, setLoading] = useState(true);


    useEffect(() => {
        let cookie = getCookie('session_id');
        verify_login(cookie)
            .then((r) => r)
            .then((data) => {
                console.log(data)
                if (data.toString() === "true") {
                    setAuthenticated("true")
                }
                setLoading(false);
            })

    }, [cookie, setAuthenticated, verify_login]);


  return (
      <>
          {(loading) ? (
              <div>loading..</div>
          ): (
              (authenticated.toString() === "true") ? (
                  <div>AUTHENTICATED!! {authenticated}</div>
              ): (
                  <div>...not authenticated...<br/>{authenticated}<br/>{cookie}</div>
              )
          )}
      </>
  );
}
