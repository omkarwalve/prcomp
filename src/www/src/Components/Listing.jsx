import React, { useEffect, useState, useRef } from 'react';
//import { BsFillInfoCircleFill } from "react-icons/bs";
import { HiOutlineInformationCircle } from "react-icons/hi"
import { ReactComponent as Filter } from "./list/filter.svg";
import { ReactComponent as RArrow } from "./list/chevron_r.svg";
import { ReactComponent as Scale } from './list/compare.svg';

import Productdef from "./Productdef";
import './Listing.css';
import './Filter.css';
import './spinner.css';
import { useLocation } from "react-router-dom";
//import axios from 'axios';

  function useQuery() {
      return new URLSearchParams(useLocation().search);
  }


const Specifications = ({specifications}) => {
  let specs = specifications;
  try {
    specs = JSON.parse(specifications);
  }
  catch(err) { return (<p className="unavail"> Unavailable </p>) }
  //console.log(specs);
  return (
    <table className="specTable">
      {
        Object.keys(specs).map(key => {
          const value = specs[key];
          return (
            <tr className='spec'>
              <td>
                {key}
              </td>
              <td style={{wordWrap:'normal'}}>
                {value}
              </td>
            </tr>
          )
        })
      }
    </table>
  )
}
const ProductDiv = ({prod}) => {

  let name = prod.name;
  if (prod.price == "Not Available") { 
    return null;
  }

  if(name.length > 110) {
    name = `${name.slice(0,110)}...`
  }

              //<h2 class="specs__title"><BsFillInfoCircleFill /></h2>
  return (
    <>    
          <div className="leftdiv">
            <div className="prod_img">
              <a href={prod.url} target="_blank" rel="noreferrer">
                <img src={prod.img} alt="error"/>
              </a>
            </div>
            <Productdef
              name={name}
              price={prod.price}
              store={"./listing/" + prod.store + ".png"}
              url={prod.url}
              warranty={prod.warranty}
              returnPolicy={prod.return_replace}
              //availibility="Availibility"
              rating={Math.floor(Math.random() * (2) ) + 3}
            />
            <div class="specs__wrapper">
              <h2 class="specs__title"><HiOutlineInformationCircle /></h2>
              <div class="specs__content">
                <p class="specs__message"><h3 className="specHead">Specifications</h3>
                  <Specifications specifications={(prod.specs)}/>
                </p>
              </div>
            </div>
            <button className="compareBtn">
              <Scale/>
            </button>
        </div>
        </>
  )
}

const FiltersMenu = ({storeSet}) => {
  const dropdownAsRef = useRef(null);
  const submenuAsRef = useRef(null);
  const [isActive, setIsActive] = useState(false);
  const [isSubActive, setSubActive] = useState(false);
  const onClick = () =>  setIsActive(!isActive);
  const onClickSub = () =>  setIsActive(!isSubActive);

  useEffect(() => {
    const pageClickEvent = (e) => {
      // Main dropdown
      if (dropdownAsRef.current !== null && !dropdownAsRef.current.contains(e.target) ) {
        setIsActive(!isActive);
        setSubActive(!isSubActive);
      }
      if (submenuAsRef.current !== null && !submenuAsRef.current.contains(e.target)) {
        setSubActive(!isSubActive);
      }
      console.log(e);
    };
    if (isActive || isSubActive) {
      window.addEventListener('click',pageClickEvent);
    }

    return() => { window.removeEventListener('click',pageClickEvent)};

  }, [isActive]);
  const stores = Array.from(storeSet);

           //<h5 className="fHead">Stores</h5>
              //<form>
              //</form>
              //<p key={store}>{store}</p>
  return (
       <div className="filterSection">
         <button className="filterButton" onClick={onClick}>
           <Filter />
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

function Listing() {
      let query = useQuery();
      const cat = query.get('cat');
      const search = query.get('search');
      const [products,setProducts] = useState([]);
      const [loading,setLoading] = useState(false);
      document.title = "kilowog(" + cat + ") =>[" + search + "]";
      const getProducts = async(url) => {
        setLoading(true);
        try{
        // ------------AXIOS -----------------
        // const res = await axios.get(url);
        // let jsonRes = res.data;
        // try{
        //   jsonRes = JSON.parse(res.data);
        // }
        // catch(err){
        //   console.log(err)
        // }
        // const result = jsonRes?.RESULTS ?? [];
        // console.log(res);
        
        // --------------FETCH---------------
        const res =  await fetch(url);
        //console.log(res)
        const response = await res.json();
        //const amazon = response?.listings[0] ?? [];
        //const flipkart = response?.listings[1] ?? [];
        //const resFinal = [...amazon,...flipkart]
        const resFinal = response?.listings ?? [];
        //console.log(resFinal)
        setLoading(false);
        setProducts(resFinal);
        }
        catch(err) { setLoading(false) }
      }

      useEffect(() => {
        const searchUrl = search.split(/\s+/).join('+');
        const reqUrl = `http://localhost:8000/${cat}/${searchUrl}`;
        //console.log(reqUrl,search);
        getProducts(reqUrl);
      },[cat,search])

      if(loading){
      //if(true){
        return (
          <div className='spinnerContainer'>
            <div className='loader'></div>
            <p className='loaderText'>Fetching..</p>
          </div>
        )
      }
      const stores = new Set();
      products.forEach((prd) => {
              stores.add(prd.store);
      });
    return (
     <div className="Listings">
       <FiltersMenu storeSet={stores}/>
       <div className='product-container'>
          {
            products.map(prod => {
              return (
                <ProductDiv prod={prod}/>
              )
            })
          }
       </div>
    </div>
    )
}

export default Listing;
