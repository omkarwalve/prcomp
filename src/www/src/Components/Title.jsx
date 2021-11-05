import React, { useEffect } from "react";

const SetTitle = ({ pageTitle }) => {
  useEffect(() => {
    const prevTitle = document.title;
    document.title = pageTitle;
    return () => {
      document.title = prevTitle;
    };
  });
  return <></>;
};

export default SetTitle;
