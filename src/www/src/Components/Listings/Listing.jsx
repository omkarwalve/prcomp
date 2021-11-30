import React, { useCallback, useEffect, useState} from 'react';

import FiltersMenu from "./Filter";
import Loading from "./Loading";
import Crashed from "./Crashed";
import Container from "./Container";
import { Compare } from "./Compare";
import './Listing.css';

import { useLocation } from "react-router-dom";
//import axios from 'axios';

function useQuery() {
    return new URLSearchParams(useLocation().search); 
}

const STORAGE_KEY = "products-cache";

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
  stringify() {
    return "{ query: " + this.query + ", results: [" + this.result + "] }" ;
  }
}

//class Cacher {
  //constructor(object_arr) {
    //let cache_arr = new Array();
    //Array(object_arr).forEach(cacheobj => cache_arr.push(new QuReCache(cacheobj.query, cacheobj.result)));
    //this.productCache = cache_arr;
  //}
//}




function storageToCacheArr() {
  let object_arr = JSON.parse(sessionStorage.getItem(STORAGE_KEY));
  if (object_arr != null && Array(object_arr).length != 0) {
    console.info("Populating runtime cacher with Session Storage cache...");
    return Array(object_arr).filter(object =>
      object != null && object.query != null && object.result != null
      ? true
      : false
      //console.log("Found null in object_arr")
    ).map(cached => 
      new QuReCache(cached.query, cached.result));
  }
  else { console.error("Empty Cacher obtained"); return []; }
}

function findCache(cacher,query) {
   return cacher.filter(cache =>
    (cache != null && cache != [] && cache.length != 0)
     ? cache.is(query)
     : false
  );
}


function Listing() {
     // # Query
     let query = useQuery();
     const cat = query.get('cat');
     const search = query.get('search');
     document.title = "kilowog(" + cat + ") =>[" + search + "]";
     // # Cache
     const [cache, setCache] = useState([]);
     //console.log("Cache var creation:- ", cache);
     //const [cacher_matches, setCacherMatches] = useState([]);
     const cacher_matches = findCache(cache,search);
     console.log("Cacher:- ",cache, "Filtered:- ", cacher_matches);
     // useState initializations
     const [products,setProducts] = useState([]);
     const [loading,setLoading] = useState(false);
     const [crashed,setCrashed] = useState(false);
     // # Stores for filter menu
     let stores = new Set();
       products.forEach((prd) => {
               stores.add(prd.store);
       });
     // # Compare window product set
     const [compareSet, setCmpProducts] = useState(new Set());
     
     // # Timeout for JS Fetch API
     const Timeout = (time) => {
          let controller = new AbortController();
          setTimeout(() => controller.abort(), time * 1000)
          return controller;
     }
     // # Async GET to query backend
     const getProducts = async(url) => {
                 setLoading(true);
                 try {
                     // Request only if query is not in cache
                     if (cache.length == 0 || cacher_matches.length == 0) {
                         console.log(search, "not found in cacher");
                         console.log("Making request...");
                         const res =  await fetch(url, { 
                           signal: Timeout(15).signal
                         });
                         //console.log(res)
                         const response = await res.json();
                         const resFinal = response?.listings ?? [];
                         //console.log(resFinal)
                         setLoading(false);
                         setProducts(resFinal);
                     }
                     else {
                       console.log("Fetching from cache...",cacher_matches[0].result);
                       setLoading(false);
                       setProducts(cacher_matches[0].result);
                     }
                 }
                 catch(err) { setLoading(false); console.error(err); setCrashed(true); }
     }

     // # On page load useEffect -- to populate cache
     useEffect(() => {
          let object_arr = JSON.parse(sessionStorage.getItem(STORAGE_KEY));
          //if (object_arr != null && Array(object_arr).length != 0) {
          if (object_arr) {
            console.info("Populating runtime cacher with Session Storage cache...");
            setCache(Array(object_arr).filter(object =>
                object != null && object.query != null && object.result != null
                ? true
                : false
                //console.log("Found null in object_arr")
              ).map(cached => 
                new QuReCache(cached.query, cached.result))
            );
          }
          else { console.warn("Empty Cacher obtained...\nInitializing cacher");
                 setCache([]); }
     }, []);

     // ===================== Update listings if category or search query changes in url ===============
     useEffect(() => {
               const searchUrl = search.split(/\s+/).join('+');
               const reqUrl = `http://localhost:8000/${cat}/${searchUrl}`;
               //console.log(reqUrl,search);
               getProducts(reqUrl);
              //setCacherMatches(cache.filter(cached => 
                //(cached != null && cached != [] && cached.length != 0)
                  //? cached.is(search) 
                  //: false
              //));
     },[cat,search])
     // ===================== Update cache when products var is updated ===================
     useEffect(() => {
       if (products.length != 0 && cacher_matches.length == 0 ) {
             setCache(c => [...c,new QuReCache(search,products)]);
         }
         else {
           console.warn("Caching skipped");
         }
     },[products]);

     // ===================== Updating Session storage with product-cache ====================
     useEffect(() => {
        if (cache.length != 0) {
           console.info("Caching to sessionStorage", cache);
           sessionStorage.setItem(STORAGE_KEY,JSON.stringify(cache));
        } else { console.warn("Cannot cache an empty cacher"); }
     },[cache]);

     //useEffect(()=> {
     //},[cache,search,cat])


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
                  <Container prod={prod} CSet={compareSet} setCSet={setCmpProducts}/>
                )
              })
            }
         </div>
      </div>
    </>
    )
}

export default Listing;
