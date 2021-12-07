
import ResultsCache from './cache';

/**
 * Assigns a value to the property if the property exists for the object.
 * Else logs an error to the console.
 * @param {object} object - The object to check in
 * @param {string} property - The property to check for
 * @param {any} value - The value to set
 */
function setIfHas(object,property,value) {
   object.hasOwnProperty(property) 
    ? object.property(value)
    : console.error(`${property} not found in ${object}`);
 }

/**
 * Checks if the property exists for the object and returns object.property .
 * Else logs an error to the console.
 * @param {object} object - The object to check in
 * @param {string} property - The property to check for
 */
function ifHas(object,property) {
   return (object.hasOwnProperty(property) 
    ? object.property
    : console.error(`${property} not found in ${object}`));
}

class Fetch {
  static #SERVER_URL = 'http://localhost:8000';

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
  static async cGET(category,query,setterz) {
    //setterz.hasOwnProperty('loading') ? setterz.loading(true) : console.error("Missing loading setState!");
    var cache = Array(ifHas(setterz,'cache'));
    var setProducts = new Function(ifHas(setterz,'products'));
    var setCache = new Function(ifHas(setterz,'setCache')) ;

    const matches = ResultsCache.filter(cache,query);
    if (matches)
      { setProducts(matches.result) }
    else { 
      const response = Fetch.GET(category,query,setterz,true) 
      setCache(cache.push(new ResultsCache(query,response)))
    }
  }

  /**
   * GET to backend server and update the setters (cache unaware).
   * @param {string} category - The chosen category
   * @param {string} query - The search query (with spaces)
   * @param {object} setterz - The object containing all the setters update on different stages of GET.
   * @param {boolean} shouldReturn - If should return the response.
   */
  static async GET(category,query,setterz,shouldReturn) {
    var setCrashed = new Function(ifHas(setterz,'crashed'));
    var setLoading = new Function(ifHas(setterz,'loading'));
    var setProducts = new Function(ifHas(setterz,'products'));
    //setIfHas(setterz,'loading',true);
    setLoading(true);
    try {
        const response = await fetch(`${Fetch.#SERVER_URL}/${category}/${query.split(/\s+/).join('+')}`
                                    , { signal: Fetch.timeout(15).signal })
                                    .then(response => response.json())
        response?.listings 
        ? setProducts(response?.listings)
        : setCrashed(true)

        return ( shouldReturn && response?.listings
                  ? response?.listings
                  : null )
    } catch(err) { 
      setLoading(false); 
      setCrashed(true); 
    }
  }

  static timeout(time) {
     let controller = new AbortController();
     setTimeout(() => controller.abort(), time * 1000)
     return controller;
  }
}

export default Fetch
