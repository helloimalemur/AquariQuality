"use client";
import Image from "next/image";
import {useEffect, useState} from "react";
import {getCookie} from "@/lib/cookies";

export default function Dashboard() {
  const [authenticated, setAuthenticated] = useState(false);
  const [cookie, setCookie] = useState("d");


    useEffect(() => {
        let cookie = getCookie('session_id');


    }, []);


  return (
      <>
        {authenticated ? (
            <div>AUTHENTICATED!!</div>
        ): (
            <div>...not authenticated...</div>
        )}

      </>

  );
}
