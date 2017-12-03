import React from "react";
import ReactDOM from "react-dom";
import ${name} from "client/components/${name_lower}";

describe("${name}", () => {
  let component;
  let container;

  beforeEach(() => {
    container = document.createElement("div");
  });

  afterEach(() => {
    ReactDOM.unmountComponentAtNode(container);
  });

  it("has expected content with deep render", () => {
    component = ReactDOM.render(
      <${name} />,
      container
    );

    expect(component).to.not.be.false;
  });
});