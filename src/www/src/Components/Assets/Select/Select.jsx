//    ___       _           _   
//   / __| ___ | | ___  __ | |_ 
//   \__ \/ -_)| |/ -_)/ _||  _|
//   |___/\___||_|\___|\__| \__|
//

// Library Imports
import React, { memo, useEffect, useRef, useState } from 'react';

// File Imports
import './select.css';


const Select = ({items,optionHolder,position}) => {

    const selectRef = useRef(null);
    const [isSelectActive, setSelectActive] = useState(false);
    const [option, setOption] = 
        useState(() => {
            var obj = Object.values(items[0]);
             return { 
                      key  : obj[0],
                      value: obj[1]
                    }
        });
    const onSelectClick = () => {
        setSelectActive(!isSelectActive); 
        //console.info("Hey Clicker!");
    }

    const changeOption = (e) => {
        setOption({ key: e.target.textContent , value: e.target.getAttribute("value") })
        //console.log("Selected Option: ", option);
        setSelectActive(false);
    }

    const pageClickEvent = (e) => {
        if (selectRef.current != null && !selectRef.current.contains(e.target) ) {
            setSelectActive(!isSelectActive);
        }
    }
    // Component Mount
    useEffect(()=>{
        //if (isSelectActive) {
            //document.addEventListener('click',pageClickEvent);
        //}
        //return () => {
            //document.removeEventListener('click', pageClickEvent);
        //}
        (isSelectActive)
            ?  document.addEventListener('click',pageClickEvent)
            :  document.removeEventListener('click', pageClickEvent)
    },[isSelectActive]);

    useEffect(()=> { optionHolder(option) },[option]);

            //{ isSelectActive && (<ul className="select-options" type="none">
    return(
        <div ref={selectRef} className={`selector-container ${ position ? position : '' }`}>
            <div className="selected-option" onClick={onSelectClick}>
                {option.key}
            </div>
            <ul className={`select-options ${isSelectActive ? 'active' : '' }`} type="none">
                {
                    items.map((item) => {
                        var obj = Object.values(item);
                        return (
                            <li className={`select-option ${isSelectActive ? 'active': '' }`}
                                value={obj[1]}
                                onClick={changeOption}>
                             {obj[0]}
                            </li> 
                        )
                })
                }
            </ul>
        </div>
    )
}

export default memo(Select)