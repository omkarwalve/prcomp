import React from 'react'

import './Catalog.css'
// All catalog logos
import { ReactComponent as Electronics } from './assets/elx.svg';
import { ReactComponent as Furniture } from './assets/fur.svg';
import { ReactComponent as Fashion } from './assets/fas.svg';
import { ReactComponent as Grocery } from './assets/gro.svg';
import { ReactComponent as Stationary } from './assets/sta.svg';
import { ReactComponent as Garden } from './assets/gar.svg';
import { ReactComponent as Toys } from './assets/toy.svg';
import { ReactComponent as Books } from './assets/bok.svg';

export default class Section extends React.Component {

                //<div className="containerRow1">
                //</div>
                //<div className="containerRow2">
                //</div>
                        //<img src="/catalog/elx.svg" alt="error" />
    render() {
        return (
            <div className="catalog">
                <div className="category-grid">
                        <div className="category-block">
                            <Electronics />
                            <div className="catName">Electronics</div>
                        </div>    
                        
                        <div className="category-block">
                            <Furniture />
                            <div className="catName">Furniture</div>                    
                        </div>

                        <div className="category-block">
                            <Fashion />
                            <div className="catName">Fashion</div>
                        </div>

                        <div className="category-block">
                            <Grocery />
                            <div className="catName">Grocery</div>
                        </div>

                        <div className="category-block">
                            <Stationary />
                            <div className="catName">Stationary</div>
                        </div>
                        
                        <div className="category-block">  
                            <Garden />
                            <div className="catName">Garden</div>
                        </div>
                        
                        <div className="category-block">                    
                            <Toys />
                            <div className="catName">Toys</div>
                        </div>
                        
                        <div className="category-block">
                            <Books />
                            <div className="catName">Books</div>
                        </div>
                </div>
            </div>
        )
    }
}
