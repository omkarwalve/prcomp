import React, { useCallback, useEffect, useState} from 'react';
import { HiOutlineInformationCircle } from "react-icons/hi"

import Productdef from "./Productdef";
import FiltersMenu from "./Filter";
import Loading from "./Loading";
import Crashed from "./Crashed";
import { Compare, CompareCheck } from "./Compare";
import './Listing.css';

import { useLocation } from "react-router-dom";
//import axios from 'axios';

function useQuery() {
    return new URLSearchParams(useLocation().search); 
}

class QuReCache {
  constructor(query, result) {
    this.query = query;
    this.result = result;
  }
  is(search_query) {
    if (this.query === search_query.trim().toLowerCase()) {
      return true;
    }
  }
}

const cached_qure = new Array();


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
          <div className="product-container">
              <a href={prod.url} target="_blank" rel="noreferrer">
                <div className="product-image">
                    <img src={prod.img} alt="error"/>
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
              </a>
            <div className="product-opts">
              <div class="specs-section">
                <h5 class="specs-icon"><HiOutlineInformationCircle /></h5>
                <div class="specs-float-section">
                  <h3 className="spec-float-title">Specifications</h3>
                    <Specifications specifications={(prod.specs)}/>
                </div>
              </div>
              <CompareCheck pid={prod.id} CSet={CSet} setCSet={setCSet}/>
            </div>
        </div>
        </>
  )
}


function Listing() {
     let query = useQuery();
     const cat = query.get('cat');
     const search = query.get('search');
     const cacher_matches = cached_qure.filter(cacher => cacher.is(search));
     console.log("Cacher:- ",cached_qure, "Filtered:- ", cacher_matches);
     const [products,setProducts] = useState([]);
     const [loading,setLoading] = useState(false);
     const [crashed,setCrashed] = useState(false);
     document.title = "kilowog(" + cat + ") =>[" + search + "]";
     const getProducts = async(url) => {
                 setLoading(true);
                 try {
                     // Cacher check

                     if (cached_qure.length == 0 || cacher_matches.length == 0) {
                         console.log(search, "not found in cacher");
                         console.log("Making request...");
                         const res =  await fetch(url);
                         //console.log(res)
                         const response = await res.json();
                         const resFinal = response?.listings ?? [];
                         //console.log(resFinal)
                         setLoading(false);
                         setProducts(resFinal);
                     }
                     else {
                       console.log("Already searched for.. Fetching from cache...")
                       console.log(cacher_matches[0].result);
                       setLoading(false);
                       setProducts(cacher_matches[0].result);
                     }
                 }
                 catch(err) { setLoading(false); setCrashed(true); }
     }

     useEffect(() => {
         console.log("Products are:",products);
       if (products.length != 0 && cacher_matches.length == 0 ) {
           cached_qure.push(new QuReCache(search,products));
         }
         else {
           //console.error("Cacher Failure!: Incorrect cacher table generation iminent");
           console.warn("Caching skipped");
         }
     },[products]);

     let stores = new Set();
       products.forEach((prd) => {
               //console.log(prd.id);
               stores.add(prd.store);
       });

     const [compareSet, setCmpProducts] = useState(new Set());

     useEffect(() => {
             //if (cached_qure.length == 0 || cacher_item == null) {
               const searchUrl = search.split(/\s+/).join('+');
               //const reqUrl = `http://localhost:8000/${cat}/${searchUrl}`;
               const reqUrl = `http://localhost:8000/${cat}/${searchUrl}`;
               //console.log(reqUrl,search);
               getProducts(reqUrl);
             //}
             //else {
               //console.log("Already searched for.. Fetching from cache...")
               //setProducts(cacher_item.result);
             //}
     },[cat,search])

     if(loading){
       return (
         <Loading />
       )
     }

     if(crashed) {
       return (
         <Crashed />
       )
     }

    return (
      <>
       <Compare products={products} CSet={compareSet}/>
       <div className="listings-section">
         <FiltersMenu storeSet={stores}/>
         <div className='products-section'>
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
