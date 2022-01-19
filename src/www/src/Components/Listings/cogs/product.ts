//  .______   .______        ______    _______   __    __    ______ .___________.
//  |   _  \  |   _  \      /  __  \  |       \ |  |  |  |  /      ||           |
//  |  |_)  | |  |_)  |    |  |  |  | |  .--.  ||  |  |  | |  ,----'`---|  |----`
//  |   ___/  |      /     |  |  |  | |  |  |  ||  |  |  | |  |         |  |     
//  |  |      |  |\  \----.|  `--'  | |  '--'  ||  `--'  | |  `----.    |  |     
//  | _|      | _| `._____| \______/  |_______/  \______/   \______|    |__|     


/**
 * `Price` - A dataclass wrapper to store price as numeric and as raw currency.
 *  @property {string} display - Raw Currency String ( e.g '₹5,699')
 *  @property {string} amount  - Numeric value       ( e.g  '5699' ) */
class Price {
  readonly display: string;
  readonly amount: number;
  static CURRENCY = ['$','₹'];
  constructor(raw: string) {
    this.display  = raw;
    this.amount   = parseFloat(raw.match(/[0-9.-]+/g)?.join('') ?? '0.0');
    //console.log(this.display,this.amount);
  }
}

/**
 * `Product` - A product dataclass for
 *  storing all product information.
 * @property {string} name           - Product Name
 * @property {string} store          - Product Store
 * @property {string} return_replace - Product Returns Policy
 * @property {string} warranty       - Product Warranty Policy
 * @property {string} specs          - Product Image URL
 * @property {string} image          - Product Image URL
 * @property {string} price          - Product Price
 * @property {string} url            - Product URL
 * @property {string} id             - Product ID               */
class Product {
    readonly name: string;
    readonly store: string;
    readonly return_replace: string | null;
    readonly warranty: string | null;
    readonly specs: object | null;
    readonly price: Price;
    readonly img: string;
    readonly url: string;
    readonly id: string;
    cloaked: boolean;

    constructor(object: any) {
      this.name = object.name;
      this.store = object.store;
      this.return_replace = (object.warranty != "❔") ? object.return_replace : null;
      this.warranty = (object.warranty != "❔") ? object.warranty : null;
      this.specs = (object.specs != "❔") ? JSON.parse(object.specs) : null;
      this.price = new Price(object.price);
      this.img = object.img;
      this.url = object.url;
      this.id = object.id;
      this.cloaked = false;
    }

    static from(object: any) : Product[] {
      return object?.listings.map((productdata: any)  => {
        return new Product(productdata);
      });
    }
    shorten(): ShortProduct {
      return new ShortProduct(this);
    }
    cloak(value?: boolean) {this.cloaked = (value) ? value : !this.cloaked; }
}


/**
 * `ShortProduct` - A mini product dataclass for
 *  places where only basal product information is required
 * @property {string} name  - Product name
 * @property {string} image - Product image
 * @property {string} price - Product price
 * @property {string} store - Product store
 * @property {string} url   - Product url
 */
class ShortProduct {
    readonly name: string;
    readonly image: string;
    readonly price: Price;
    readonly store: string;
    readonly url: string;

    constructor(product: Product) {
        this.name  = product.name;
        this.image = product.img;
        this.price = product.price;
        this.store = product.store;
        this.url   = product.url;
    }
}

export { ShortProduct }
export default Product;