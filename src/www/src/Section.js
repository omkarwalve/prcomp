import React from 'react'
import './Section.css'

export default class Section extends React.Component {

    render() {
        return (
            <div className="content">

                <div className="containerRow1">
                    
                    <div className="electronics">
                        <a href="#">
                        <img src="/electronic-category.jpg" alt="error" />
                        <div className="catName">Electronics</div></a>
                    </div>    
                    
                    <div className="furniture">
                    <a href="#">
                        <img src="/furniture-category.jpg" alt="error" />
                        <div className="catName">Furniture</div></a>                    
                    </div>

                    <div className="fashion">
                    <a href="#">
                        <img src="/fashion-category.jpg" alt="error" />
                        <div className="catName">Fashion</div></a>
                    </div>

                    <div className="grocery">
                    <a href="#">
                        <img src="/grocery-category.jfif" alt="error" />
                        <div className="catName">Grocery</div></a>
                    </div>

                </div>

                <div className="containerRow2">

                    <div className="stationary">
                    <a href="#">
                        <img src="/stationary-category.jpg" alt="error" />
                        <div className="catName">Stationary</div></a>
                    </div>
                    
                    <div className="garden">
                    <a href="#">
                        <img src="/garden-category.jpg" alt="error" />
                        <div className="catName">Garden</div></a>
                    </div>
                    
                    <div className="toys">
                    <a href="#">
                        <img src="/toys-category.jpg" alt="error" />
                        <div className="catName">Toys</div></a>
                    </div>
                    
                    <div className="books">
                    <a href="#">
                        <img src="/books-category.jpg" alt="error" />
                        <div className="catName">Books</div></a>
                    </div>
                
                </div>

            </div>

        )
    }
}
