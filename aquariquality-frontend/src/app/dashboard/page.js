"use client";
import Image from "next/image";
import {useState} from "react";

export default function Home() {
  const [authenticated, setAuthenticated] = useState(false);
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
