import { Page } from "../App";

interface Props {
  type: Page;
}

function List(props: Props) {
  return (
    <div class="container">
      <h2>{props.type.valueOf()}</h2>
    </div>
  );
}

export default List;
