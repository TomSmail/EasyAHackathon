import styled from "styled-components";

const SearchBarWrapper = styled.div`
  height: 100px;
  display: flex;
  justify-content: center;
  align-items: center;
`;

const SearchBarContainer = styled.div`
  display: flex;
  align-items: center;
  max-width: 400px;
  background-color: #fff;
  border-radius: 25px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  padding: 5px;
`;

const SearchIcon = styled.span`
  color: #aaa;
  margin: 0 10px;
`;

const SearchInput = styled.input`
  border: none;
  flex-grow: 1;
  padding: 10px;
  font-size: 16px;

  &:focus {
    outline: none;
  }
`;

const styles = {
  SearchBarWrapper,
  SearchBarContainer,
  SearchIcon,
  SearchInput,
};

export default styles;
