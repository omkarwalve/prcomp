
import { Options } from './Menu';
import Product, { PriceModifiers, Sort, SortOption, SortOptions } from 'Components/Listings/cogs/product';

/** ### Filters Discrete Menu [*Small screens*]
 * Filter Menu options for screens less than 992px wide(mobile).*/
const TPanel = ({filters}: {filters: Options[]}) => {
  return(
        <ul className="filter-options">
            {
                filters.map(filter => {
                  return(
                    <li className="filter-option">
                      {filter.icon}
                      {filter.name}
                    </li>
                  )
                })
            }
        </ul>
  )
}

// # Filter Modal
interface ModalProps {
  filter: SortOption;
}
const Modal = ({filter}: ModalProps) => {
  return (
    <span className="filter-modal">
    </span>
  )
}

export default TPanel;