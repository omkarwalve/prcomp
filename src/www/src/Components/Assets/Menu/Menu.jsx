//                        
//   _____                
//  |     | ___  ___  _ _ 
//  | | | || -_||   || | |
//  |_|_|_||___||_|_||___|
//                        

// Library Imports
import React, { memo, useEffect, useState, useRef} from 'react';

// File Imports
import './menu.css';

//const Menu = ({parentRef,items}) => {
const Menu = ({menuSwitch,items}) => {
    /* Prop `items` format
     * items = { 
     *           text: "Menu Text",
     *           uri : "www.google.com" 
     *         }
    */

    //const [isMenuActive,setMenuActive] = useState(false);
    //const menuRef = useRef(null);
    //console.log("Ref:- ", parentRef ,"Typeof Ref.current:- " + typeof parentRef.current);
    //parentRef.current.onclick(onParentClick);
    //parentRef.current.actions.onclick(onParentClick);
    //const onParentClick = (e) => {
        //console.log("Before:- ",isMenuActive);
        //setMenuActive(!isMenuActive);
        //console.log("After:- ", isMenuActive,"Parent got clicked");
    //}

    //useEffect(() => {
        // Set parent's onClick function
        //parentRef.current.onclick = onParentClick;
        //console.log("ParentRef",parentRef);
        //console.log("menuRef",menuRef.current.parentNode)
        //menuRef.current.parentNode.onclick = onParentClick;

        //const pageClickEvent = (e) => {
            //if (parentRef.current != null && !parentRef.current.contains(e.target) ) {
                //setMenuActive(!isMenuActive);
            //}
        //}
        //if (isMenuActive) {
            //document.addEventListener('click',pageClickEvent);
        //}
        //return () => {
            //document.removeEventListener('click', pageClickEvent);
        //}
    //}, []);

    //useEffect(() => {
        //console.log("Menu", isMenuActive);
    //}, [isMenuActive]);

                                //<a href={uri} rel="no-referrer" { target != "_self" || target == null ? `target=${target}`: ''}>
    return(
        <div className={`menu-container ${menuSwitch ? 'active' : 'inactive'}`}>
            <ul type="none" className="menu-list">
                {
                    items.map(({text,uri,action}) => {
                        return(
                            <li className="menu-option">
								{(action) 
									? ( <button rel="no-referrer" onClick={action}> {text} </button>) 
									: ( <a href={uri} rel="no-referrer"> {text} </a>) 
								}
                            </li>
                        )
                    })
                }
            </ul>
        </div>
    )
}

export default memo(Menu);
