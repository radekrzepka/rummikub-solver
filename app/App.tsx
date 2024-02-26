import { StatusBar } from "expo-status-bar";
import { StyleSheet, Text, View } from "react-native";
import { styled } from "nativewind";

const StyledText = styled(Text);
const StyledView = styled(View);

export default function App() {
  return (
    // <View style={styles.container}>
    <StyledView className=" flex-1 items-center content-center justify-center">
      <StyledText className="">
        Radek jest{" "}
        <StyledText className="uppercase font-bold">Czarny</StyledText>
      </StyledText>
      <StatusBar style="auto" />
    </StyledView>
  );
}

// const styles = StyleSheet.create({
//   container: {
//     flex: 1,
//     backgroundColor: '#fff',
//     alignItems: 'center',
//     justifyContent: 'center',
//   },
// });
