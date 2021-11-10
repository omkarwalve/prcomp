import React, { useCallback, useEffect, useState} from 'react';
import { HiOutlineInformationCircle } from "react-icons/hi"

import Productdef from "./Productdef";
import FiltersMenu from "./Filter";
import { Compare, CompareCheck } from "./Compare";
import './Listing.css';
import './spinner.css';
import { useLocation } from "react-router-dom";
//import axios from 'axios';

  function useQuery() {
      return new URLSearchParams(useLocation().search); }


const Specifications = ({specifications}) => {
  let specs = specifications;
  try {
    specs = JSON.parse(specifications);
  }
  catch(err) { return (<p className="unavail"> Unavailable </p>) }
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
const ProductDiv = ({prod,CSet,setCSet}) => {

  //let name = prod.name;
  if (prod.price === "Not Available") { 
    return null;
  }

  //if(name.length > 110) {
    //name = `${name.slice(0,110)}...`
  //}

  return (
    <>    
          <div className="leftdiv">
            <div className="prod_img">
              <a href={prod.url} target="_blank" rel="noreferrer">
                <img src={prod.img} alt="error"/>
              </a>
            </div>
            <Productdef
              name={prod.name}
              price={prod.price}
              store={"./listing/" + prod.store + ".png"}
              url={prod.url}
              warranty={prod.warranty}
              returnPolicy={prod.return_replace}
              //availibility="Availibility"
              rating={Math.floor(Math.random() * (2) ) + 3}
            />
            <div class="specs__wrapper">
              <h5 class="specs__title"><HiOutlineInformationCircle /></h5>
              <div class="specs__content">
                <h3 className="specHead">Specifications</h3>
                  <Specifications specifications={(prod.specs)}/>
              </div>
            </div>
            <CompareCheck pid={prod.id} CSet={CSet} setCSet={setCSet}/>
        </div>
        </>
  )
}


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

      let stores = new Set();
        products.forEach((prd) => {
                //console.log(prd.id);
                stores.add(prd.store);
        });

      const [compareSet, setCmpProducts] = useState(new Set());
      //const [compareSet, setCmpProducts] = useState([]);
      //const adder = useCallback((product) => setCmpProducts(plist=> plist.add(product)));
      //const remover = useCallback((product) => setCmpProducts(plist=> plist.delete(product)));

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

    return (
      <>
       <Compare products={products} CSet={compareSet}/>
       <div className="Listings">
         <FiltersMenu storeSet={stores}/>
         <div className='product-container'>
            {
              products.map(prod => {
                return (
                  <ProductDiv prod={prod} CSet={compareSet} setCSet={setCmpProducts}/>
                )
              })
            }
         </div>
      </div>
    </>
    )
}

export default Listing;
