//  .____     .__           __   .__                     __                  
//  |    |    |__|  _______/  |_ |__|  ____    ____    _/  |_  _________  ___
//  |    |    |  | /  ___/\   __\|  | /    \  / ___\   \   __\/  ___/\  \/  /
//  |    |___ |  | \___ \  |  |  |  ||   |  \/ /_/  >   |  |  \___ \  >    < 
//  |_______ \|__|/____  > |__|  |__||___|  /\___  / /\ |__| /____  >/__/\_ \
//          \/         \/                 \//_____/  \/           \/       \/

// Library Imports
import React, { useEffect, useState, useReducer, useMemo, createContext, useCallback } from 'react';
import { useLocation } from 'react-router-dom';
import Compare, { cmpReducer } from './Components/Compare/Compare';

// Cogs Imports
import ResultsCache from './cogs/cache';
import Fetch from './cogs/fetch';
import Product, { ShortProduct } from './cogs/product';
import { cartActions } from 'Components/Assets/Cart/Cart';
// Component Imports
import Card from './Components/Card/Card';
import Filter from './Components/Filter/Wrapper';
import Crash from 'Components/Assets/Crash/Crash';
import Loading from 'Components/Assets/Loading/Loading';

// CSS
import './listing.css';
import Sort, { SortOptions } from './cogs/sort';
import useToggle from 'hooks/toggle';

// Custom Hook
function useQuery() {
    return new URLSearchParams(useLocation().search);
}

export const Veily = createContext<[boolean,() => void]>([false,() => {}]);

const Listing = () => {
  let query = useQuery();
  const category = query.get('cat');
  const search = query.get('search');
  document.title = "kilowog(" + category + ") =>[" + search + "]";

  const [veiled,toggleVeil] = useToggle(false);
  const [cache,setCache] = useState<ResultsCache[]>(ResultsCache.retrieve());
  const [products,setProducts] = useState<Product[]>();
  const [loading,setLoading] = useState<boolean>(false);
  const [crashed,setCrashed] = useState<boolean>(false);
  const [filters,setFilters] = useState<SortOptions>();
  const setFiltersCallback = useCallback(setFilters,[]);
  const filterProducts = useCallback(() => (filters) && setProducts( pdx => (pdx) ? Sort.sort([...pdx],filters) : undefined ),[filters]);

  const [stores,setStores] = useState<Set<string>>(new Set());
  const storesMemo = useMemo(() => {
    if (products) { 
      let _stores: Set<string> = new Set();
      products.forEach(product => {
        _stores.add(product.store);
      });
      setStores(_stores);
    }
    return stores;
  // }, [category,search]);
  }, [products]);

  const [layout, setLayout] = useState<string>("compact");

  const [compareSet,setCompare] = useReducer(cmpReducer,new Set());
  const GET = useCallback(async () => {
    // Fetch.GET(
    // Fetch.cGET(
    Fetch.mockGET(
      category ?? '',
      search ?? '',
      {
        cache: cache,
        crashed: setCrashed,
        loading: setLoading,
        products: setProducts,
        setCache: setCache,
      },
      // false
    );
  },[category,search]);

  //# On `category`/ `search` update do...
  useEffect(()=> {
    //# Fetch Products
    GET();
    //! Apply filters on product
    filterProducts();
  }, [category,search]);

    //! Apply filters on product
  useEffect(() => filterProducts() , [filters]);

  // useEffect(()=> {
  //   ResultsCache.store(cache);
  // },[cache]);

  // useEffect(()=> { (compareSet.size) && console.log("Detected change in compareSet"); },[compareSet]);

  //# On `products` update..
  // useEffect(()=> {
  //   //# Update Stores
  // },[products]);

  switch(true) {
    case (loading):   return ( <Loading /> ) 
    case (crashed):   return ( <Crash /> ) 
    default: 
      return (
          <>
          <Veily.Provider value={[veiled,toggleVeil]}>
          {/* { 
            (products) 
              ? ( */}
              <div className='toolkit-panel'>
                   <Filter setFLT={setFiltersCallback} lyt={layout} setLYT={setLayout} stores={storesMemo} />
                   {(products) && <Compare products={products} compareSet={compareSet} />}
                 </div>

                {/* )
            : <> </>
          } */}
          <div className="product-section">
            {
              products && products.map(product => {
                return (
                  <Card key={product.id} product={product} layout={layout} setCmp={setCompare} />
                )
              })
            }
          </div>
          </Veily.Provider>
          </>
        )
  }
}

export default Listing
