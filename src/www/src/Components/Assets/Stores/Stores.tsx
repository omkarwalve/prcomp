import React,{useCallback} from 'react';
import { Item } from '../List/list';
import { ReactComponent as Amazon }      from './assets/Amazon.svg';
import { ReactComponent as Flipkart }    from './assets/Flipkart.svg';
import { ReactComponent as Urbanladder } from './assets/Urbanladder.svg';
import { ReactComponent as NullStore }   from 'assets/nullstore.svg';
import './storeicon.css';

interface StoreProperties {
  SVG: ({ store }: { store: string; }) => JSX.Element;
  PNG: ({ store }: { store: string; }) => JSX.Element;
  Item:(store: string) => Item;
}
const Store: StoreProperties = { 
  SVG: ({store}: {store: string}): JSX.Element => {
    const stores: Map<string, JSX.Element> = new Map([
      ["amazon"      , <Amazon      className='store-icon-vectored' />],
      ["flipkart"    , <Flipkart    className='store-icon-vectored' />],
      ["urbanladder" , <Urbanladder className='store-icon-vectored' />],
    ]);
    return ( (stores.get(store.toLowerCase()) ?? <NullStore className='store-icon-vectored' />) )
  }
, PNG: ({store}: {store: string}) => {
    const capitalizeFirst = (str: string): string => { return str.charAt(0).toUpperCase() + str.slice(1); }
    return (
      <img src={`/stores/${capitalizeFirst(store.toLowerCase())}.png`} onError={(e) => e.currentTarget.src = "/stores/nullstore_black.png" }  className='store-icon' alt='Error Loading Store Logo' />
      )
  }
, Item: (store: string): Item => {
  const noStoreItem: Item = { key: ":)", value: ":)", conceal: false }
  const storeItems: Map<string,Item> = new Map([
    ["amazon",      { key: "amazon"     , value: <Store.PNG store='amazon'     />, conceal: false }],
    ["flipkart",    { key: "flipkart"   , value: <Store.PNG store='flipkart'   />, conceal: false }],
    ["urbanladder", { key: "urbanladder", value: <Store.PNG store='urbanladder'/>, conceal: false }]
  ])
  return storeItems.get(store.toLowerCase()) ?? noStoreItem;
  }
};


export default Store;