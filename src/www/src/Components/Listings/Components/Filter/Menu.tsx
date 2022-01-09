//   _______  _  _                    
//  (_______)(_)| | _                 
//   _____    _ | || |_   ____   ____ 
//  |  ___)  | || ||  _) / _  ) / ___)
//  | |      | || || |__( (/ / | |    
//  |_|      |_||_| \___)\____)|_|    
//                                    

// Library Imports
import React, { useState, useEffect, useRef, useMemo, useContext } from 'react';
// Outside Click Custom Hook
import useOnClickOutside from 'hooks/useOnClickOutside';

import { Viewport } from 'App';
// SVG
import { ReactComponent as Funnel } from './assets/filter.svg'

import { ReactComponent as Money } from './assets/filter-money.svg'
import { ReactComponent as Storeicon } from './assets/filter-store.svg'
import { ReactComponent as Brandicon } from './assets/filter-brand.svg'
import { ReactComponent as Keywordicon } from './assets/filter-keyword.svg'

import { ReactComponent as Flow } from './assets/lyt-flow.svg'
import { ReactComponent as Compact } from './assets/lyt-compact.svg'
import { ReactComponent as Big } from './assets/lyt-big.svg'
import { ReactComponent as Versus } from './assets/lyt-versus.svg'

// Cogs
import Product, { PriceModifiers, Sort, SortOption, SortOptions } from 'Components/Listings/cogs/product';
import { Price, Store, Brand, Keyword } from './Filters';
import Panel from './Panel';
import TPanel from './TPanel';

// CSS
import './menu.css';
import useToggle from 'hooks/toggle';
import useSET from 'hooks/set';


// Icon-Options-List Type Interface
export interface Options { 
  name: string; 
  icon: JSX.Element;
  target?: JSX.Element;
};

// # Layout Selector
const LayoutSelect = ({lyt,setLYT}: {lyt: string, setLYT: FilterProps['setLYT'] }) => {
  const Layouts: Options[] = [
    { name: "big"    , icon: <Big />     },
    { name: "compact", icon: <Compact /> },
    { name: "flow"   , icon: <Flow />    },
    { name: "versus" , icon: <Versus />  },
  ];
  const onLayoutClick = (e: React.MouseEvent<HTMLLIElement>) => {
    e.preventDefault();
    var selected = e.currentTarget.getAttribute("value") ;
    (selected) 
      ? setLYT(selected) 
      : console.error("No attribute `value` for layout-option")
  }
  return (
    <ul className="layout-options">
      {
        Layouts.map(layout => {
          return (
            <li 
              className={`layout-option ${(layout.name == lyt) ? 'selected': ''}`}
              value={layout.name}
              onClick={onLayoutClick}>
              {layout.icon}
            </li>
          )
      })
      }
    </ul>
  )
}

interface FilterProps {
    pdx: Product[] | undefined;
    setPDX: React.Dispatch<React.SetStateAction<Product[] | undefined>>;
    lyt: string;
    setLYT: React.Dispatch<React.SetStateAction<string>>;
}
/**  # Filter Menu
 * Component for sorting `Listings` by keywords or properties.
 * Also controls `Layout` of `Listings`.  */
const Filter = ({pdx,setPDX,lyt,setLYT}: FilterProps) => {
  const [width] = useContext(Viewport);
  const [isFilterActive,toggleFilter] = useToggle(false);
  const filterRef = useRef<HTMLDivElement>(null);
  // useOnClickOutside(filterRef,toggleFilter,isFilterActive);
  const onHamburgerClick = () => { toggleFilter(); }
  useEffect(()=>{
    console.info(isFilterActive);
    //(isFilterActive) 
        //? document.addEventListener('click',pageClickEvent)
        //: document.removeEventListener('click', pageClickEvent)
},[isFilterActive]);

  // Filter states
  const [prflt,setPrflt] = useState<PriceModifiers>();
  const [kwrd,_,setKwrds] = useSET<string>();

  const filters: Options[] = [ 
    { name: "Price"  , icon: <Money       /> , target: <Price prflt={prflt} setPrflt={setPrflt} /> },
    { name: "Store"  , icon: <Storeicon   /> , target: <Store /> },
    { name: "Brand"  , icon: <Brandicon   /> , target: <Brand /> },
    { name: "Keyword", icon: <Keywordicon /> , target: <Keyword setKeywordlist={setKwrds} />},
  ];
  // Products brute sort
  useMemo(() => {
    // var sopt: SortOptions = {price: {order: "HL", rng: {min: 1000, max: 55000}}, priority: [SortOption.price]};
    // var sopt: SortOptions = { price: 'HL', priority: [SortOption.price] };
    console.log(prflt);
    var sopt: SortOptions = { price: prflt , priority: [SortOption.price] };
    setPDX( pdx => (pdx) ? Sort.sort(pdx,sopt) : undefined );
  }, [pdx,prflt]);

  return (
    <div className='filter-container'>
      <button className={`filter-hamburger-trigger ${isFilterActive ? 'open' : '' }`} onClick={onHamburgerClick}>
        <Funnel />
      </button>
      {/* {(isFilterActive) 
      && ( */}
        <div ref={filterRef} className={`filter-menu ${isFilterActive ? 'active' : '' }`}>
          <h3>Filter</h3>
          <hr />
          {(width < 992) 
          && (<TPanel filters={filters} />)
          || (<Panel  filters={filters}/>)
          }
          <hr />
          <LayoutSelect lyt={lyt} setLYT={setLYT} />
        </div>
      {/* )} */}
    </div>
  )
}

export default Filter;
