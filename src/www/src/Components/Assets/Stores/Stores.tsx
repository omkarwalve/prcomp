import React from 'react';
import { ReactComponent as Amazon }      from './assets/Amazon.svg';
import { ReactComponent as Flipkart }    from './assets/Flipkart.svg';
import { ReactComponent as Urbanladder } from './assets/Urbanladder.svg';

import './storeicon.css';

const STORE$icon = ({store,vectored = true}: {store: string, vectored?: boolean}) => {
  const stores: Map<string, JSX.Element> = new Map([
    ["amazon"      , <Amazon      className='store-icon-vc' />],
    ["flipkart"    , <Flipkart    className='store-icon-vc' />],
    ["urbanladder" , <Urbanladder className='store-icon-vc' />],
  ]);
  const capitalizeFirst = (str: string): string => {
    return str.charAt(0).toUpperCase() + str.slice(1);
  }

  return (
    vectored 
    ? (stores.get(store.toLowerCase()) ?? <></>)
    : <img src={`/stores/${capitalizeFirst(store.toLowerCase())}.png`}  className='store-icon' alt='error' />
    )
}

export default STORE$icon;
