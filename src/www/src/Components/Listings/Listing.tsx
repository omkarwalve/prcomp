//  .____     .__           __   .__                     __                  
//  |    |    |__|  _______/  |_ |__|  ____    ____    _/  |_  _________  ___
//  |    |    |  | /  ___/\   __\|  | /    \  / ___\   \   __\/  ___/\  \/  /
//  |    |___ |  | \___ \  |  |  |  ||   |  \/ /_/  >   |  |  \___ \  >    < 
//  |_______ \|__|/____  > |__|  |__||___|  /\___  / /\ |__| /____  >/__/\_ \
//          \/         \/                 \//_____/  \/           \/       \/

// Library Imports
import React, { useEffect, useState, useReducer } from 'react';
import { useLocation } from 'react-router-dom';
import Compare, { cmpReducer } from './Components/Compare/Compare';

// Cogs Imports
import ResultsCache from './cogs/cache';
import Fetch from './cogs/fetch';
import Product, { ShortProduct } from './cogs/product';
import { cartActions } from 'Components/Assets/Cart/Cart';
// Component Imports
import Card from './Components/Card/Card';
import Filter from './Components/Filter/Menu';
import Crash from 'Components/Assets/Crash/Crash';
import Loading from 'Components/Assets/Loading/Loading';

// CSS
import './listing.css';

// Custom Hook
function useQuery() {
    return new URLSearchParams(useLocation().search);
}

const Listing = ({setCart}: {setCart: React.Dispatch<cartActions>}) => {
  let query = useQuery();
  const category = query.get('cat');
  const search = query.get('search');
  document.title = "kilowog(" + category + ") =>[" + search + "]";

  const [cache,setCache] = useState<ResultsCache[]>(ResultsCache.retrieve());
  const [products,setProducts] = useState<Product[]>();
  const [loading,setLoading] = useState<boolean>(false);
  const [crashed,setCrashed] = useState<boolean>(false);

  const [stores,setStores] = useState<Set<string>>(new Set());

  const [layout, setLayout] = useState<string>("compact");

  const [compareSet,setCompare] = useReducer(cmpReducer,new Set());


  //# On `category`/ `search` update do...
  useEffect(()=> {
    //# Fetch Products
    //Fetch.cGET(
    Fetch.mockGET(
      category ?? '',
      search ?? '',
      {
        cache: cache,
        crashed: setCrashed,
        loading: setLoading,
        products: setProducts,
        setCache: setCache,
      }
    )
  }, [category,search]);

  useEffect(()=> {
    ResultsCache.store(cache);
  },[cache]);

    useEffect(()=> {
        console.log("Detected change in compareSet");
    },[compareSet]);

  //# On `products` update..
  useEffect(()=> {
    //# Update Stores
    if (products) { 
      let _stores: Set<string> = new Set();
      products.forEach(product => {
        _stores.add(product.store);
      });
      setStores(_stores);
    }
    console.log(products);
  },[products]);

  switch(true) {
    case (loading):   return ( <Loading /> ) 
    case (crashed):   return ( <Crash /> ) 
    default: 
      return (
          <>
          { 
            (products) 
              ? (<>
                   <Filter pdx={products} setPDX={setProducts} lyt={layout} setLYT={setLayout} />
                   <Compare products={products} compareSet={compareSet} />
                 </>
                )
            : <> </>
          }
          <div className="product-section">
            {
              products && products.map(product => {
                return (
                  <Card product={product} layout={layout} setCmp={setCompare} setCartItems={setCart} />
                )
              })
            }
          </div>
          </>
        )
  }
}

export default Listing
