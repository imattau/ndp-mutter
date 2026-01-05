use zbus::{Connection, Proxy, Result};
use zbus::zvariant::{Value, ObjectPath, OwnedObjectPath};
use std::collections::HashMap;

pub struct MutterScreenCast<'a> {
    proxy: Proxy<'a>,
}

impl<'a> MutterScreenCast<'a> {
    pub async fn new(connection: &Connection) -> Result<Self> {
        let proxy = Proxy::new(
            connection,
            "org.gnome.Mutter.ScreenCast",
            "/org/gnome/Mutter/ScreenCast",
            "org.gnome.Mutter.ScreenCast",
        )
        .await?;
        Ok(Self { proxy })
    }

    pub async fn create_session(&self) -> Result<MutterSession<'a>> {
        let args = HashMap::<&str, Value>::new(); // Empty properties for now
        let path: OwnedObjectPath = self.proxy.call("CreateSession", &(args)).await?;
        MutterSession::new(self.proxy.connection(), path.into()).await
    }
}

pub struct MutterSession<'a> {
    proxy: Proxy<'a>,
}

impl<'a> MutterSession<'a> {
    pub async fn new(connection: &Connection, path: ObjectPath<'a>) -> Result<Self> {
        let proxy = Proxy::new(
            connection,
            "org.gnome.Mutter.ScreenCast",
            path,
            "org.gnome.Mutter.ScreenCast.Session",
        )
        .await?;
        Ok(Self { proxy })
    }

    pub async fn record_virtual(&self, width: u32, height: u32) -> Result<MutterStream<'a>> {
        let mut props: HashMap<&str, Value> = HashMap::new();
        props.insert("is-virtual", Value::Bool(true));
        props.insert("width", Value::U32(width));
        props.insert("height", Value::U32(height));
        
        let path: OwnedObjectPath = self.proxy.call("RecordVirtual", &(props)).await?;
        MutterStream::new(self.proxy.connection(), path.into()).await
    }

    pub async fn start(&self) -> Result<()> {
        self.proxy.call("Start", &()).await
    }
}

pub struct MutterStream<'a> {
    proxy: Proxy<'a>,
}

impl<'a> MutterStream<'a> {
    pub async fn new(connection: &Connection, path: ObjectPath<'a>) -> Result<Self> {
        let proxy = Proxy::new(
            connection,
            "org.gnome.Mutter.ScreenCast",
            path,
            "org.gnome.Mutter.ScreenCast.Stream",
        )
        .await?;
        Ok(Self { proxy })
    }

    pub async fn pipewire_node_id(&self) -> Result<u32> {
        self.proxy.get_property("PipeWireNodeId").await
    }
}
