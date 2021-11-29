import React, { useState, useEffect, useRef } from "react";
import './Filter.css';

import { ReactComponent as FilterI } from "./assets/filtermenu/filter.svg";
//import { ReactComponent as RArrow } from "./list/chevron_r.svg";

const FiltersMenu = ({storeSet}) => {
  const dropdownAsRef = useRef(null);
  const submenuAsRef = useRef(null);
  const [isActive, setIsActive] = useState(false);
  const [isSubActive, setSubActive] = useState(false);
  const onClick = () =>  { setIsActive(!isActive); setSubActive(false);};
  const onClickSub = () =>  setSubActive(!isSubActive);

  useEffect(() => {
    const pageClickEvent = (e) => {
      // Main dropdown
      if (dropdownAsRef.current !== null && !dropdownAsRef.current.contains(e.target) ) {
        setIsActive(!isActive);
      }
      if (submenuAsRef.current !== null && !submenuAsRef.current.contains(e.target)) {
        setSubActive(!isSubActive);
      }
      console.log(e);
    };
    if (isActive && isSubActive) {
      window.addEventListener('click',pageClickEvent);
    }

    return() => { window.removeEventListener('click',pageClickEvent)};

  },[isActive, isSubActive]);
  const stores = Array.from(storeSet);

           //<h5 className="fHead">Stores</h5>
              //<form>
              //</form>
              //<p key={store}>{store}</p>
  return (
       <div className="filterSection">
         <button className="filterButton" onClick={onClick}>
           <FilterI />
         </button>
         <nav ref={dropdownAsRef} className={`filterMenu ${isActive ? 'active' : 'inactive'}`}>
           <h5 className="fHead">Filter by..</h5>
           <button className="fCriteria" onClick={onClickSub}>Store </button>
           <nav ref={submenuAsRef} className={`subMenu ${isSubActive ? 'active' : 'inactive'}`}>
             <h6 className="subHead">Stores</h6>
           {
               stores.map((store) => {
               return (
                 <label name="Store" className="filterChkLabel">
                    <input type="checkbox" value={store} name="Store" className="filterChkbx"></input>
                   {store}
                 </label>
               )
           })
           }
           </nav>
           <button className="fCriteria">Price </button>
         </nav>
       </div>
  );
};

export default FiltersMenu;
