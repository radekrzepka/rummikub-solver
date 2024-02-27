import {
  View,
  Text,
  Button,
  Image,
  StyleSheet,
  SafeAreaView,
} from "react-native";
import { styled } from "nativewind";
import { Camera, CameraType } from "expo-camera";
import * as ImagePicker from "expo-image-picker";
import { shareAsync } from "expo-sharing";
import * as MediaLibrary from "expo-media-library";
import { useState, useEffect, useRef } from "react";
import { StatusBar } from "expo-status-bar";

const StyledText = styled(Text);
const StyledView = styled(View);

export default function CameraPage() {
  let cameraRef = useRef();
  const [hasCameraPermission, setHasCameraPermission] = useState(null);
  const [hasMediaLibraryPermission, setHasMediaLibraryPermission] =
    useState(null);
  const [picture, setPicture] = useState();

  useEffect(() => {
    const getPermissions = async () => {
      const { status: cameraStatus } =
        await Camera.requestCameraPermissionsAsync();
      const { status: mediaLibraryStatus } =
        await MediaLibrary.requestPermissionsAsync();
      setHasCameraPermission(cameraStatus === "granted");
      setHasMediaLibraryPermission(mediaLibraryStatus === "granted");
    };
    getPermissions();
  }, []);

  if (hasCameraPermission === null || hasMediaLibraryPermission === null) {
    return (
      <StyledView>
        <StyledText>Requesting permissions...</StyledText>
      </StyledView>
    );
  } else if (!hasCameraPermission || !hasMediaLibraryPermission) {
    return (
      <StyledView>
        <StyledText>Camera and/or media library permissions denied.</StyledText>
      </StyledView>
    );
  }

  let takePicture = async () => {
    try {
      let options = {
        quality: 1,
        base64: true,
        exif: false,
      };
      let newPhoto = await cameraRef.current.takePictureAsync(options);
      setPicture(newPhoto);
    } catch (error) {
      console.error("Error taking picture: ", error);
    }
  };

  if (picture) {
    let sharePicture = () => {
      shareAsync(picture.uri).then(() => {
        setPicture(undefined);
      });
    };
    let savePicture = () => {
      MediaLibrary.saveToLibraryAsync(picture.uri).then(() => {
        setPicture(undefined);
      });
    };
    return (
      <SafeAreaView>
        <Image source={{ uri: "data:image/jpg;base64," + picture.base64 }} />
        <Button title="Share" onPress={sharePicture} />
        {hasMediaLibraryPermission ? (
          <Button title="Save" onPress={savePicture} />
        ) : undefined}
        <Button title="Discard" onPress={() => setPicture(undefined)} />
      </SafeAreaView>
    );
  }

  return (
    <Camera ref={cameraRef} style={{ flex: 1 }}>
      <StyledView className="flex-1 items-center justify-center">
        <Button title="Take Picture" onPress={takePicture} />
      </StyledView>
      <StatusBar style="auto" />
    </Camera>
  );
}
