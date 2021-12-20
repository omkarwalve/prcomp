//                      
//  ,---.          |    
//  |    ,---.,---.|--- 
//  |    ,---||    |    
//  `---'`---^`    `---'
//                      

// Library Imports
import React, { useEffect, useState } from 'react';
import { ShortProduct } from 'Components/Listings/cogs/product';
import Tooltip from 'Components/Assets/Tooltip/Tooltip';

// SVG
import { ReactComponent as CartAdd } from './assets/add.svg';
import { ReactComponent as CartIcon } from './assets/cart.svg';

// CSS
import './cart.css';

//## `cartReducer` for cart items manipulation ------------------
export interface cartActions {
    do: 'add' | 'delete';
    product: ShortProduct;
}
function cartReducer(state: Set<ShortProduct | unknown>, action: cartActions): Set<ShortProduct|unknown> {
    switch (action.do) {
        case 'add':
            return state.add(action.product);
        case 'delete':
            return state.delete(action.product) && state || state;
        default:
            return state
    }
}

//## Add to cart button ------------------
interface AddProps {
    product: ShortProduct;
    setCart: React.Dispatch<cartActions>;
}
/** `Add to Cart Button` */
const Add = ({product,setCart}: AddProps) => {
    const [isAdded, toggleAdd] = useState<boolean>(false);
    const onAddClick = (e: React.MouseEvent<HTMLButtonElement>) => { e.stopPropagation(); toggleAdd(!isAdded);}
    useEffect(()=>{ 
        (isAdded) 
            ? setCart({do: 'add'   , product: product})
            : setCart({do: 'delete', product: product})
    },[isAdded]);
    return (
        <button className={`cart-add-button ${isAdded ? 'checked' : '' }`} onClick={onAddClick}>
            <CartAdd />
        </button>
    )
}

//## Cart items button ------------------
/** `Navbar Cart Button` */
const Cart = ({cart}: {cart: Set<ShortProduct| unknown>}) => {
    return (
        <span className="cart-nav">
            <CartIcon />
            <Tooltip text={`${cart.size} items in cart`} />
        </span>
    )
}

export { Add, cartReducer };
export default Cart;
