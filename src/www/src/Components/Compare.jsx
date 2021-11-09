import React, { useState, useEffect, useRef } from "react";

import { ReactComponent as Scale } from './list/compare.svg';

import './Compare.css';

let CompareProducts = new Set();

const CompareCheck = ({pid}) => {
    const [isCmpActive, setCmpActive] = useState(false);
    const onCbtnClick = () => setCmpActive(!isCmpActive);
    useEffect(() => {
        if (isCmpActive) {
            CompareProducts.add(pid);
        }
        else { CompareProducts.delete(pid) }
        console.log(CompareProducts);
    },[isCmpActive]);
    return (
        <>
            <div className={`compareBtn ${isCmpActive ? 'check': ''}`} onClick={onCbtnClick}>
              <Scale/>
            </div>
        </>
    )
};

const Compare = () => {
    let [len, setLen] = useState(CompareProducts.size);
    useEffect(()=> {
        setLen(CompareProducts.size);
        console.log(len);
    },[JSON.stringify(CompareProducts)]);
    return (
        <>
            <div className="compareWindow">
                <h6>{len} items to compare..</h6>
            </div>
        </>
    )
};

export { Compare, CompareCheck };
