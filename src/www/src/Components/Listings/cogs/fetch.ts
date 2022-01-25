
import React from 'react';
import ResultsCache from './cache';
import Product from './product';


// Cached product feed
import sample_json from './razer_laptop.json';

// /**
//  * Assigns a value to the property if the property exists for the object.
//  * Else logs an error to the console.
//  * @param {object} object - The object to check in
//  * @param {string} property - The property to check for
//  * @param {any} value - The value to set
//  */
//function setIfHas(object : object ,property: string,value : unknown) {
   //object.hasOwnProperty(property) 
    //? object.property(value)
    //: console.error(`${property} not found in ${object}`);
 //}

// /**
//  * Checks if the property exists for the object and returns object.property .
//  * Else logs an error to the console.
//  * @param {object} object - The object to check in
//  * @param {string} property - The property to check for
//  */
//function ifHas(object,property) {
   //return (object.hasOwnProperty(property) 
    //? object.property
    //: console.error(`${property} not found in ${object}`));
//}


interface setters {
    cache: ResultsCache[],
    crashed: React.Dispatch<React.SetStateAction<boolean>>,
    loading: React.Dispatch<React.SetStateAction<boolean>>,
    products: React.Dispatch<React.SetStateAction<Product[] | undefined>>,
    setCache: React.Dispatch<React.SetStateAction<ResultsCache[]>>,
  }

class Fetch {
  static #SERVER_URL = 'http://localhost:8000';
  static #TIMEOUT = 30;
  static #FETCH_OPTIONS: RequestInit = {
    method: 'GET',
    mode: 'cors',
    credentials: 'omit',
    headers: { 'Content-Type': 'application/json' },
    redirect: 'follow',
    // referrerPolicy: 'origin-when-cross-origin',
    signal: Fetch.timeout(Fetch.#TIMEOUT).signal
  };

  /**
   * Cache aware GET to backend server and update the setters.
   * If query is found in cache then no GET is made to server instead setter is updated with cached data
   * @param {string} category - The chosen category
   * @param {string} query - The search query (with spaces)
   * @param {object} setterz - The object containing all the setters update on different stages of GET.
   * setterz is of the type:
   * {
   *   cache,
   *   crashed,
   *   loading,
   *   products,
   *   setCache,
   * }
   */
  static async cGET(category : string,query: string,setterz: setters) {
    //setterz.hasOwnProperty('loading') ? setterz.loading(true) : console.error("Missing loading setState!");
    var cache = setterz.cache;
    var setProducts = setterz.products;
    var setCache = setterz.setCache;

    const matches = ResultsCache.filter(cache,query);
    if (matches)
      { setProducts(matches[0].result) }
    else { 
      const response = await Fetch.GET(category,query,setterz,true);
      setCache((cache) => {
        cache.push(new ResultsCache(query,response));
        return cache;
      })
    }
  }

  /**
   * Fill mock data into products
   * @param {string} category - The chosen category
   * @param {string} query - The search query (with spaces)
   * @param {object} setterz - The object containing all the setters update on different stages of GET.
   */
  static async mockGET(_ : string,__: string,setterz: setters) {
    var setProducts = setterz.products;
    setterz.loading(true);
    console.info("SAMPLE_JSON::RAW - ",sample_json);
    const parsedPDX = Product.from(sample_json);
    console.info("SAMPLE_JSON::PARSED - ",parsedPDX);
    setProducts(parsedPDX);
    await Fetch.sleep(2);
    setterz.loading(false);
  }

  static async sleep(ms: number) { 
    return new Promise((resolve)  => setTimeout(resolve,ms * 1000));
  }

  /**
   * GET to backend server and update the setters (cache unaware).
//   * @param {string} category - The chosen category
//   * @param {string} query - The search query (with spaces)
//   * @param {object} setterz - The object containing all the setters update on different stages of GET.
//   * @param {boolean} shouldReturn - If should return the response.
   */
  static async GET(category: string,query: string,setterz: setters,shouldReturn: boolean) {
    var setCrashed  = setterz.crashed ;
    var setLoading  = setterz.loading ;
    var setProducts = setterz.products;
    //setIfHas(setterz,'loading',true);
    setLoading(true);
    var reqOpts = {
      method: "GET",
      headers: { "Content-Type": "application/json" }
    };
    try {
        const response = await fetch(`${Fetch.#SERVER_URL}/q/${category}/${query.split(/\s+/).join('+')}`
                                    , Fetch.#FETCH_OPTIONS)
                                    .then(response => { console.log("GET:- ",response); return response.json(); });
        if (response?.listings) {
         setLoading(false);
         console.log("response?.listings:- ",response?.listings);
         setProducts(Product.from(response));
        } else { setCrashed(true) }

        return ( shouldReturn && response?.listings
                  ? response?.listings
                  : null )
    } catch(err) {
      console.error(err);
      setLoading(false);
      setCrashed(true);
    }
  }

  static timeout(time: number) {
     let controller = new AbortController();
     setTimeout(() => controller.abort(), time * 1000)
     return controller;
  }
}

export default Fetch