use bytes::Buf;
use quick_xml::de;
use serde::Deserialize;
fn main() {
    let bs = bytes::Bytes::from(
        r#" 
        <?xml version="1.0" encoding="utf-8"?>
        <EnumerationResults ServiceEndpoint="https://d2lark.blob.core.windows.net/" ContainerName="myazurebucket">
            <Blobs>
                <Blob>
                    <Name>dir1/2f018bb5-466f-4af1-84fa-2b167374ee06</Name>
                    <Properties>
                        <Creation-Time>Sun, 20 Mar 2022 11:29:03 GMT</Creation-Time>
                        <Last-Modified>Sun, 20 Mar 2022 11:29:03 GMT</Last-Modified>
                        <Etag>0x8DA0A64D66790C3</Etag>
                        <Content-Length>3485277</Content-Length>
                        <Content-Type>application/octet-stream</Content-Type>
                        <Content-Encoding />
                        <Content-Language />
                        <Content-CRC64 />
                        <Content-MD5>llJ/+jOlx5GdA1sL7SdKuw==</Content-MD5>
                        <Cache-Control />
                        <Content-Disposition />
                        <BlobType>BlockBlob</BlobType>
                        <AccessTier>Hot</AccessTier>
                        <AccessTierInferred>true</AccessTierInferred>
                        <LeaseStatus>unlocked</LeaseStatus>
                        <LeaseState>available</LeaseState>
                        <ServerEncrypted>true</ServerEncrypted>
                    </Properties>
                    <OrMetadata />
                </Blob>
                <Blob>
                    <Name>dir1/b2d96f8b-d467-40d1-bb11-4632dddbf5b5</Name>
                    <Properties>
                        <Creation-Time>Sun, 20 Mar 2022 11:31:57 GMT</Creation-Time>
                        <Last-Modified>Sun, 20 Mar 2022 11:31:57 GMT</Last-Modified>
                        <Etag>0x8DA0A653DC82981</Etag>
                        <Content-Length>1259677</Content-Length>
                        <Content-Type>application/octet-stream</Content-Type>
                        <Content-Encoding />
                        <Content-Language />
                        <Content-CRC64 />
                        <Content-MD5>AxTiFXHwrXKaZC5b7ZRybw==</Content-MD5>
                        <Cache-Control />
                        <Content-Disposition />
                        <BlobType>BlockBlob</BlobType>
                        <AccessTier>Hot</AccessTier>
                        <AccessTierInferred>true</AccessTierInferred>
                        <LeaseStatus>unlocked</LeaseStatus>
                        <LeaseState>available</LeaseState>
                        <ServerEncrypted>true</ServerEncrypted>
                    </Properties>
                    <OrMetadata />
                </Blob>
            </Blobs>
            <NextMarker>helloworld</NextMarker>
        </EnumerationResults>"#,
    );
    let out: Output = de::from_reader(bs.reader()).expect("must success");

    println!("{:?}", out);
}

// #[derive(Default, Debug, Deserialize)]
// #[serde(default, rename_all = "PascalCase")]
// struct Output {
//     blobs:Blob,
// }

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct Output {
    blobs:Blobs,
    #[serde(rename = "NextMarker")]
    nextmarker:Option<String>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct Blobs {
    blob:Vec<Blob>,

}
#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct Blob {
    properties: Properties,
    name: String,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct Properties {
    #[serde(rename = "Content-Length")]
    content_length: u64,
}

