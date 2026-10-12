CGE Risk
IncidentXP Software Manual
For IncidentXP 10.4 | Revision 23 (17-Sep-2021)




                                                  Please note that this documentation is preliminary and subject
                                                  to change without notice. The latest version of this document
                                                  can be obtained via CGE Risk (e-mail support@cgerisk.com) or
                                                  via an IncidentXP Value Added Reseller.
                                               Please note that this documentation is preliminary and
                                               subject to change without notice. The latest version of
                                               this document can be obtained via CGE Risk (e-mail
                                               support@cgerisk.com) or via an IncidentXP Value Added
                                               Reseller.



                                               Copyright

                                               © CGE Risk Management Solutions B.V. 2004-2021.
                                               BowTieXP, IncidentXP and AuditXP are registered
                                               trademarks. Subject to change without notice. All rights
                                               reserved.

                                               BSCAT (Barrier based Systemic Cause Analysis
                                               Technique) is property of Det Norske Veritas Ltd. and
                                               copyright © 2011 Det Norske Veritas Ltd. BSCAT is a
                                               registered trademark of DNV. The BSCAT method
                                               manual is copyright © 2011 Det Norske Veritas Ltd. All
                                               other trademarks and brands are the property of their
                                               respective owners.

                                               The Tripod Beta manual ‘Tripod Beta: Guidance on
                                               using Tripod Beta in the investigation and analysis of
                                               incidents, accidents and business losses’ is copyright ©
                                               2014 of the Stichting Tripod Foundation, and published
                                               by the Energy Institute. The Stichting Tripod
                                               Foundation ‘Tripod’ logo is registered trademark of the
                                               Stichting Tripod Foundation.

                                               TOP-SET is a registered trademark of the Kelvin
                                               Consultants Ltd.




© WOLTER S KLUWER | CG E R isk 2021   INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 2
                                             MANUAL
   TABLE OF CONTENTS

   TABLE OF CONTENTS
   1     INTRODUCTION                                                  6
   1.1    Purpose of this document                                     6
   1.2    Structure of this document                                   6


   2     QUICK INSTALLATION GUIDE                                      7
   2.1    Introduction                                                 7
   2.2    Installing IncidentXP                                        7
   2.3    Activating IncidentXP                                        7


   3     QUICK GETTING STARTED GUIDE                                  10
   3.1    The IncidentXP screen                                       10
   3.2    Creating an incident & incident analysis diagram            13
   3.3    The Incident Manager                                        14
   3.4    Creating a timeline                                         21


   4     THE BSCAT METHOD                                             26
   4.1    A brief description of BSCAT methodology                    26
   4.2    Creating a BSCAT diagram                                    28
   4.3    Add a BSCAT event                                           29
   4.4    Add a BSCAT incident barrier                                32
   4.5    Add a BSCAT causation assessment                            34
   4.6    The Connector                                               36
   4.7    Edit the BSCAT diagram                                      36


   5     THE TRIPOD BETA METHOD                                       37
   5.1    A brief description of Tripod Beta methodology              37
   5.2    Creating a Tripod Beta diagram                              39
   5.3    Add a Tripod trio (Event – Agent – Object)                  40
   5.4    Add a Tripod Beta incident barrier                          42
   5.5    Add a Tripod Beta causation assessment                      45
   5.6    The Connector                                               47
   5.7    Editing your Tripod Beta diagram                            48


   6     THE BARRIER FAILURE ANALYSIS (BFA) METHOD                    48
   6.1    A brief description of BFA methodology                      48
   6.2    Creating a BFA diagram                                      49
   6.3    Add a BFA event                                             50
   6.4    Add a BFA incident barrier                                  53
   6.5    Add a BFA causation assessment                              54
   6.6    The Connector                                               56
   6.7    Edit the BFA diagram                                        56



© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                    PAGE 3
                                                   MANUAL
   7      THE ROOT CAUSE ANALYSIS (RCA) METHOD                            58
   7.1     A brief description of RCA methodology                         58
   7.2     Creating an RCA diagram                                        58
   7.3     Add an RCA event                                               59
   7.4     The Connector                                                  61
   7.5     Edit the RCA diagram                                           61
   7.6     Link RCA event to equivalent incident diagram                  62


   8      KELVIN TOP-SET ROOT CAUSE ANALYSIS                              63
   8.1     A brief description of RCA methodology                         63
   8.2     Creating a Kelvin TOP-SET diagram                              64
   8.3     Add an TOP-SET RCA event                                       65
   8.4     Add an TOP-SET RCA event in parallel to another event          67
   8.5     The Connector                                                  67
   8.6     Edit the TOP-SET RCA diagram                                   68
   8.7     Link TOP-SET RCA event to equivalent incident diagram          68


   9      ACTIONS / RECOMMENDATIONS                                       70
   9.1     Actions as a TODO items                                        70
   9.2     Actions as recommendations                                     70
   9.3     Adding an action                                               71
   9.4     Visualizing actions                                            73
   9.5     Action reports                                                 74

   10      LINKING INCIDENTS WITH BOWTIE                                  74
   10.1      Creating a bowtie templated incident analysis diagram        75
   10.2      Linking incident analysis elements to bowtie diagrams        77
   10.3      Visualizing incident analysis data on the bowtie diagram     79


   11      CONFIGURING THE DIAGRAM                                        82
   11.1      Changing display settings for a single type of item          83
   11.2      Changing display settings for the whole diagram              84
   11.3      Adjusting lookup tables                                      87
   11.4      Layout grid                                                  89


   12      CASE FILE TOOLS                                                90
   12.1      Listview window                                              90
   12.2      Quality checks window                                        91
   12.3      Case overview                                                91


   13      LINKING TO DOCUMENTATION                                       93
   13.1      Introduction                                                 93
   13.2      Creating a document link                                     94
   13.3      Linking a document link to the diagram                       95


   14      GETTING DATA OUT OF THE PROGRAM                                95
   14.1      Introduction                                                 95



© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                        PAGE 4
                                                     MANUAL
   14.2    Reporting                                                         96
   14.3    Export of diagrams                                               100


   15     A BRIEF DESCRIPTION OF A SUBSET OF BOWTIE METHODOLOGY             101
   15.1    Hazard                                                           101
   15.2    Top event                                                        101
   15.3    Threats                                                          102
   15.4    Consequences                                                     102
   15.5    Barriers (also known as controls)                                102
   15.6    Escalation factors/defeating factors/barrier decay mechanisms    102
   15.7    Terminology recap                                                102


   16     SUPPORT                                                           103
   16.1    Helpdesk                                                         103
   16.2    Software training                                                103
   16.3    BSCAT method training                                            103
   16.4    Tripod Beta method training                                      103
   16.5    TOP-SET method training                                          103




© WOLTER S KLUWER | CG E R isk 2021       INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                           PAGE 5
                                                 MANUAL
   1
   INTRODUCTION
   1.1      PURPO SE O F TH IS D O CUMENT
   The purpose of this software manual is to provide an explanation of the IncidentXP software tool,
   which supports both DNV-GL’s BSCAT technique for incident investigation as well as the Tripod Beta
   technique for incident analysis. It assumes basic familiarity with accident and incident investigation
   methods.

   For BSCAT: This manual combined with the DNV-GL BSCAT Method Manual should be sufficient for
   experienced investigators to use the method and software successfully.

   For Tripod: Familiarity, or preferably being an accredited Tripod professional, along with this manual,
   should be enough to get you going with the software.

   For BFA & RCA: This manual should be sufficient for experienced investigators to use the software
   successfully.

   Training courses in BSCAT and Tripod Beta are available in the market. Customized training for the
   BSCAT method is available through DNV-GL (www.dnvgl.com). Training for Tripod Beta is available
   through various parties, please contact the Stichting Tripod Foundation or the energy institute
   (https://publishing.energyinst.org/tripod). For training in BFA, please contact CGE Risk at
   support@cgerisk.com.


   1.2      STRUCTUR E O F TH IS D O CUMENT
   1. Step by step guides to get you started:
        • Chapter 2, Quick installation guide: This guide quickly describes how to install and activate
           IncidentXP on your computer.
        • Chapter 3,
           Quick getting started guide: This guide briefly discusses how to work through an incident
           analysis. The configuration of timeline diagrams is also discussed in this chapter.

   2. The four incident analysis methodologies available in IncidentXP:
       • Chapter 4,
           The BSCAT method.
       • Chapter 5, The Tripod Beta method.
       • Chapter 6,
           The Barrier Failure Analysis (BFA) method.
       • Chapter 7, The Root Cause Analysis (RCA) method.

   3. Recommendations and feedback to bowtie risk assessment:
       • Chapter 9, Actions / recommendations.
       • Chapter 10, Linking incidents with bowtie.

   4. Advanced functionality; Display settings, document links, export of data and bowtie methodology
   description:




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 6
                                                   MANUAL
       •   Chapter 11,
           Configuring the diagram.
       •   Chapter 12, Case file tools.
       •   Chapter 13,
           Linking to documentation.
       •   Chapter 14, Getting data out of the program.
       •   Chapter 15,
           A brief description of a subset of bowtie methodology.

   5. Support options:
       • Chapter 16,
           Support.




   2
   QUICK INSTALLATION GUIDE
   2.1       INTRO D UCTI O N
   Installing IncidentXP onto your computer is very simple. If, however, you run into trouble and need
   more detailed information about the installation procedure or the software prerequisites and
   compatibility, then this information is available in the full BowTieXP software manual.

   Note: BowTieXP and IncidentXP are licensed modules within the same software application. Your
   license key will determine which module you will be able to access:
        • IncidentXP,
        • BowTieXP
        • or both.

   This is the reason why you will download the IncidentXP software from the BowTieXP downloads
   site.


   2.2       INSTALLING INCID ENTX P
       Note: IncidentXP can run on Windows 7 and newer. IncidentXP requires the .NET framework v4.7 or v4.8 to be
       installed. If you run BowTieXP and the .NET framework is too old, it BowTieXP tell you so. You can then download
       and install it:

       https://dotnet.microsoft.com/download/dotnet-framework/thank-you/net48-web-installer

       Note that Windows 10 Creators Update (1703) and later come with a new enough version of the framework already
       installed out of the box.


   Open an internet browser and navigate to: http://www.cgerisk.com/downloads/bowtiexp/
   Download and run the BowTieXP installer (.msi).


   2.3       ACTIV ATING INCID ENTX P
   When you start IncidentXP for the first time, you are asked for either a trial code or an activation
   code:




© WOLTER S KLUWER | CG E R isk 2021                INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                    PAGE 7
                                                          MANUAL
                                          Figure 1 - Activation dialog
   If you have a valid trial code, you can enter it in the activation or trial code text box, and IncidentXP
   will run.

   If you have purchased IncidentXP, you will need to obtain an activation code to perpetually activate
   IncidentXP. Please copy the code shown in the current host ID text box and paste it into an email to
   CGE Risk at support@cgerisk.com. We will then send you an activation code. After entering this code
   in the activation or trial code text box, the current version of IncidentXP will run on your
   desktop/laptop forever.

   2.3. 1          Activating with an old host -id
   Sometimes, due to the use of a different laptop docking station, a switch in a Windows version, or
   changes in the computer hardware, it can happen that your host ID changes. If that happens, your
   activation code will no longer match with the host ID. If this happens, you will have to activate the
   software based on the original host-ID (the original host-ID that was used when creating the
   activation code, so the one you sent to the CGE Risk support staff).

   To do so, select the “Lock to this computer with an old host id” option from the “Lock to” drop-down
   menu in the Product Activation screen (see Figure 2). The screen will then display a field for the
   current, as well as the activated host-id (see Figure 3).




                                    Figure 2 - Activate with original host id




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                       PAGE 8
                                                     MANUAL
                        Figure 3 – Host id and activation code do not match anymore


   We do not need to send you a new code. You just need to tell it the host id which belongs to the
   activation code. You can do this by copy/pasting the old host id which was used to generate the
   activation code into the “Activated host id” field. The host id that was used to generate the activation
   code is always included in the same email as the activation code, usually in a bit of text that looks like
   Figure 4:




                                   Figure 4 - Activation code email message


   The host-id that was used to generate the activation code is the one pointed out by the arrow in
   Figure 4. When this host-id is copied into the “Activated host id’ field, the software will pick it up and
   then match it to the activation code. This should result in an accepted code, returning the status
   “OK”. Now click the OK button, and the software should be activated once more.

   Please note that IncidentXP will check what has changed on your computer. If too many parts have
   changed, or you are trying this on a different computer, the code will still not work, and you will need
   to contact us for an updated code.




                                          Figure 5 – Changed host-id




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                       PAGE 9
                                                     MANUAL
   3
   QUICK GETTING STARTED GUIDE
   3.1      TH E INCID ENTX P S CREEN
   After starting IncidentXP, the following screen will appear:




                                      Figure 6 - Main application screen


   This screen consists of several parts that you will need to familiarize yourself with before you will be
   able to start building analysis diagrams.

   Along the top we find the menu bar and the main toolbar.




                                     Figure 7 - Menu bar & main toolbar


   Below those we have the diagram and the treeview.


   3.1. 1          The diagram
   The biggest portion of the screen is taken up by the diagram.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 10
                                                    MANUAL
                                        Figure 8 - Diagram window


   The diagram window is where the visual portion of your diagram will take shape, think of the
   diagram window as your drawing board.

   The diagram can show the timeline of an incident, the incident analysis diagram, or both. The
   analysis diagram can be either a BSCAT, Tripod, BFA or RCA diagram. When you start the software
   and no Incidents have been created yet, the diagram window will either show you the Case
   Overview, the Incident Manager, or an empty Incident Diagram sheet.


   3.1. 2          The tre e vie w
   The treeview is normally located on the left-hand side of the screen. This can be adjusted according
   to your preferences. The treeview displays all the information present in your case file and helps you
   navigate through your case file. Think of the treeview as your case file manager.




                                        Figure 9 - Treeview window


   Through this window you have access to all information in your file, such as the incidents itself, but
   also reference information such as
       • The incident categorizations such as types and tiers (see Incident Lookup Tables),
       • Other reference information such as document links and
       • Method-specific reference information such as the BSCAT chart items or Tripod BRF lists (also
            Incident Lookup Tables).


   3.1. 3          The e ditor
   In an editor window you can submit and edit all written content within your case file.

   When you want to add a new item, you enter the item’s details into this window. When you want to
   change an item, you also have to do it in this window.

   By double clicking items in the treeview or in your diagram window, the editor window will appear
   which allows you to edit the various fields and reference data of an item.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 11
                                                   MANUAL
                            Figure 10 - Edit windows for an event item or tree view item


   Along the top we have different tabs. The leftmost one is called ‘General’ and this is where we have
   all the basic data which is available for an item. In these examples, we are looking at items that have,
   amongst other properties, a name, and a description.

   The next tab is called ‘User Data’ and it is where you can define your own fields, if so required. Please
   see the BowTieXP software manual for more information regarding this tab.

   The tab ‘Document Links’ will be discussed in chapter 13
   Linking to documentation.


       Some tips if you get lost:

       Pressing F2 allows you to edit the name of the selected item.
       Pressing F4 will bring up the editor window for the selected item.

       If you can’t find your diagram, you either have not selected an incident, or you have selected a different tab such
       as the case overview tab instead of the diagram tab.



   3.1. 4             Add and re move buttons
   The software allows you to remove buttons from the diagram toolbar and only display the buttons
   you frequently use. In the original software settings, some buttons will be hidden by default.

   You can remove (hide) or add buttons from/to the diagram toolbar by clicking on the three tiny
   buttons with three small dots and an arrow on it.




                              Figure 11 - Add or remove buttons from the toolbar (1/2)


   Check the toolbar buttons that you want to visualize in the toolbar and uncheck the buttons that
   should be hidden.




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                      PAGE 12
                                                            MANUAL
                              Figure 12 - Add or remove buttons from the toolbar (2/2)


   3.2  CREATING AN INCID ENT & INCID ENT ANALYSIS
   DIAGRAM
   When you start IncidentXP, the software automatically creates a blank file for you.
       Note: The way IncidentXP works with files is very similar to how you open and save files in Microsoft Word. Similarly,
       you can attach your case files to your e-mail messages, transfer them to an USB stick, etc.

   To create a new incident in the software, right-click the incidents node in the treeview and
   subsequently select ‘New Incident’. The editor will pop up allowing you to specify the name, date,
   incident type, category, and tier. In this window you should also decide which analysis method you
   want to use.
       Note: Depending on which methods are activated/licensed in the software, you will be offered a choice of method.
       If only one method is activated, that method is automatically selected.




                             Figure 13 - Adding a new incident diagram in the treeview


   After creating the new incident, the treeview presents us one or multiple empty diagrams
   underneath the incident node. These diagrams can be either incident analysis diagrams or timeline
   diagrams.




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                        PAGE 13
                                                            MANUAL
              Figure 14 - An incident node which contains a Timeline diagram and BFA Diagram


   In chapter 3 we will explain how to create the timeline, and thereafter in separate chapters discuss
   creation of BSCAT diagrams, Tripod Beta diagrams, BFA diagrams, and RCA diagrams.


   3.3       TH E INCID ENT MANAGER
   Incident investigation can be chaotic at times, with different steps involved that are often being
   attended to interchangeably. The Incident Manager helps to guide you through the steps of incident
   investigation, providing you a more structured approach. From top to bottom there are 6 tabs
   present within the Incident Manager: Details, Investigation, Analysis, Recommendations,
   Attachments and Report. We believe these 6 tabs are best followed in this proposed order; however,
   the software allows you to switch back and forth as you see fit.

   You can open the Incident Manager by going to ‘View > Windows > Incident Manager’, or use the
   hotkey Ctrl + Shift + J. Alternatively, if it has already been activated before, you can click on ‘incident
   manager’ in your diagram tabs, as illustrated in Figure 15 by the red circle.




                            Figure 15 – The start screen of the Incident Manager


   Once you’ve built multiple incidents, the start screen of your Incident Manager will look different. It
   will contain an overview of all the incidents within your case file, displayed as tiles. Clicking on one
   will directly take you to that particular incident.




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                       PAGE 14
                                                     MANUAL
               Figure 16 – The start screen of the Incident Manager holding multiple incidents


   As an alternative way to create an incident you can click on ‘Add Incident’ within the Incident
   Manager to create a new incident. In this example, we used ‘Conveyer belt fire’ as a fictional incident
   taken from our test file. After creating a new incident and filling in its basic details, your screen will
   look like this:




                             Figure 17 – The Details tab of the Incident Manager


   3.3. 1  Walking through the 6 tabs of the Incide nt
   Manager
   The Incident Manager consists of 6 tabs, which can be walked through in any order:
       • Details
       • Investigation
       • Analysis
       • Recommendations
       • Attachments
       • Report




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                     PAGE 15
                                                     MANUAL
   Details
   The ‘Details’ tab gives you an overview of the information you have entered when creating a new




   incident (see

   Figure 17 – ).




   There are 4 grey buttons near the top of this page as shown in the image below.




                    Figure 18 – The 4 buttons of the Details tab of the Incident Manager


   The first button will open the ‘Edit Incident windows’ and allows you to change the details of the
   incident. The second button allows you to add new actions, which can be viewed under
   ‘Recommendations’. The third button allows you to add a new incident diagram. The fourth button
   completely deletes the incident from your case file. It will give you a warning before being able to
   delete an incident. Moreover, you can undo it using the undo button (or Ctrl + Z).
   In vesti ga ti o n
   The ‘Investigation’ tab contains a reference to your timeline diagram. See chapter 3.4 ‘Creating a
   timeline’ for more information on the timeline. Left click on ‘Timeline Diagram’ to go to the timeline
   diagram.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 16
                                                    MANUAL
                          Figure 19 – The Investigation tab of the Incident Manager


   Note that there is an add new timeline diagram button on this page as well.




                                   Figure 2020 - Add new timeline diagram


   Analysis
   The analysis tab gives an overview of all the analysis diagrams you have of an incident. Left clicking
   on, for example, ‘Top Set RCA’ (or one of the other incident techniques), will take you to that
   respective incident diagram.

   Note that there is an add new incident diagram button on this page as well.




                                    Figure 21 - Add new incident diagram


   It is also possible to click on the pen on paper icon to change the original name of the incident and
   set its status to ‘Completed’.




                                   Figure 22 - Edit incident diagram details


   For more information on the different incident analysis techniques, please see chapter 4, 5, 6, 7, or 8.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 17
                                                    MANUAL
   Reco mmen d a t i o n s
   The ‘Recommendations’ tab gives an overview of all the actions concerned with the incident;
   completed as well as not completed. This list shows you both actions that belong to an Incident
   Diagram/Timeline shape as well as actions that do not belong to any specific element, but are instead
   related to the overall incident, diagram or timeline. The column “Belongs to” gives you this
   information. In this example, we have one action related to a diagram shape called ‘Eroded wiring,
   while the other 3 actions are not related to any shape. Clicking on the shape in the ‘Belongs to’
   column, will take you directly to that specific element in the incident/timeline diagram.

   Additionally, there are three buttons on this page (as shown in red). The first button lets you add
   additional general actions, i.e. actions that are not related to any shape of the incident/timeline
   diagram. The second button allows you to export your action list to Excel. The third button only
   shows up if you select an action and allows you to delete the selected action.




                        Figure 22 – The Recommendations tab of the Incident Manager


   For more information on ‘Recommendations’, please see chapter 9.


   A t t a ch men t s
   The ‘Attachments’ tab gives an overview of all the documents attached to this incident. For example,
   it is possible to add picture evidence, interview reports, building blueprints etc. The software allows
   for almost any type of file to be attached, from .jpeg to .pdf. ‘Attachments’ differs from ‘Document
   Links’: Be careful not to attach too many large documents, as this will add to the file size of your
   bowtie file. Clicking on the paper with paperclip button will allow
    you to add attachments.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 18
                                                    MANUAL
                          Figure 23 – The Attachments tab of the Incident Manager


   Report
   The ‘Report’ tab gives an overview of the reports of an incident. There are 5 buttons on this page (as
   shown in red).




                                1     2     3     4     5




                             Figure 24 – The Report tab of the Incident Manager


   The first button will allow you to choose a template for your report. You can choose between the
   built-in template (see Figure 26) or a custom made one. The first page of the built-in template can be
   seen in
   Figure 26. Templates use predefined tags that draw their information from IncidentXP to
   automatically fill in a report. For more information on how this works see chapter 14.2.2.

   The second button creates a report based on the template we defined using the first button. Left
   click it to save it on your computer, and it will automatically open a word document. The third button
   uploads existing documents. For example, you can link the report you just made using the first two
   buttons. The fourth button refreshes the report that was saved on the computer. This is to ensure
   that the linked report you made using the third button can be updated when you have updated your
   existing incident analysis in IncidentXP.

   The fifth button only shows up if you’ve selected a report and allows you to unlink the selected
   report, so you will not be able to see it in the reports list for updating. Note: There will be no warning
   nor can this be recovered by clicking “Undo” or Ctrl + Z.




© WOLTER S KLUWER | CG E R isk 2021             INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                     PAGE 19
                                                       MANUAL
                             Figure 25 – Choosing what report template to use




                               Figure 26 – Example of default built in report


   The settings icon     will take you to a pop-up screen that will allow you to choose what diagram to
   use (you can choose only one) and what timeline diagrams to involve (you can choose multiple) in
   that particular report with every update.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 20
                                                   MANUAL
   3.4       CREATING A TIMELINE
   The timeline allows you to enter all evidence/events and visually reorder them to fit your findings.
   The software automatically creates a basic timeline diagram from which you can start working
   immediately.




                                      Figure 27 - Default start of timeline


   To add a timeline diagram to an existing incident, right-click an incident in the treeview and select
   ‘New Incident Diagram’. Select methodology ‘Timeline’ in the add incident diagram dialog and give
   the timeline a name.




                               Figure 28 - Add new timeline to existing incident


   In this window you will also be able to select the ‘Diagram Status’, which is linked to a functionality in
   BowTieServer. You can choose to ignore this when you are not a BowTieServer user. You can filter
   out the data of draft incident diagrams on bowtie diagrams by means of the incident filter when
   using IncidentXP in combination with BowTieXP.

   3.4. 1          Add a time line column
   To add a new timeline column, move your mouse over the right- or left-hand side of an existing
   timeline column and click the green plus button which appears.




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                     PAGE 21
                                                     MANUAL
                                           Figure 29 – Add timeline column


   Select a date and time and a date/time format. Subsequently select ‘OK’ to add the column. A
   qualitative date and time can optionally be inserted to highlight a particular moment with a
   qualitative description (e.g. First Deviation) that will be shown at the top of the column.




                                      Figure 30 – Enter timeline column details


   Another way of adding a timeline column is to drag an existing timeline column to the right or left
   side of the diagram.
       Note: A red exclamation mark will appear when timeline columns are not in chronological order.




© WOLTER S KLUWER | CG E R isk 2021                INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                        PAGE 22
                                                          MANUAL
                    Figure 31 - Exclamation mark in the timeline (non-chronological order)


   3.4. 2           Add a time line actor

       Note: The term actor can refer to a person but also to an item, machine, system, or other item which can cause
       actions (or influence the flow of events).


   To add a new timeline actor, move your mouse over the top (or bottom) of an existing timeline actor
   and click the green plus button that appears. Add a name in the field that appears and press ‘Enter’.




                                            Figure 32 - Add timeline actor


   3.4. 3           Add a timeline event
   To add a new timeline event, move your mouse over one of the cells (where the time column and the
   actor row cross each other) and click the green plus button.




© WOLTER S KLUWER | CG E R isk 2021               INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                PAGE 23
                                                         MANUAL
                                          Figure 33 - Add a new timeline event


   Double click a timeline event to specify the name, description, actor, location, category and
   confidence level. See next paragraph for more information on editing different elements.




                                           Figure 34 - Specify a timeline event


       Note: There will be a new actor row added to the timeline diagram when a new actor is created in the timeline
       event editor. An actor can be added in the timeline event editor by selecting this icon



   3.4. 4            Edit timeline
   We can open the editor of the time column, actor row or event by double clicking them, or by
   clicking the pencil icon on an element  . For communicative purposes it is also possible to change
   colors of elements or entire columns.

   We can also drag timeline events to other positions within the diagram. Dropping it onto a different
   cell will implicitly alter the timestamp and actor accordingly.




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                               PAGE 24
                                                            MANUAL
                                      Figure 35 - Example of a timeline


   Unused actors will be hidden from the diagram by clicking this button         in the toolbar. If you
   cannot find this button in the toolbar you have to add it first to the toolbar using the ‘add or remove
   buttons’, see chapter 3.1.4.




                                 Figure 36 – Timeline without unused actors


   3.4. 5          Copy/pas te of time line e ve nts to othe r diagrams
   It is also possible to copy an event from the timeline and paste it directly into an analysis diagram.
   This applies to all methods within IncidentXP. Copy/pasting can be done via the ‘traditional’ way
   (using ctrl+C on a timeline event and using ctrl+V in an analysis diagram). Additionally, you can drag
   and drop an event while holding the ALT key. Lastly, you can drag and drop through the Tree view.
   Make sure to drop an event as a ‘child’ on the correct ‘parent’ (the diagram name) item.

   Copying an event from an analysis diagram to the timeline is also possible. Make sure that you select
   the header of a timeline column to paste an element into the timeline. It will automatically snap onto
   the NULL or No Value Assigned row at the bottom of the timeline.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                   PAGE 25
                                                    MANUAL
   4
   THE BSCAT METHOD
   4.1       A BRIEF D ESCRIPTIO N O F BSCAT METH O D O LO GY


       Note: Refer to DNV-GL’s BSCAT method manual for a more thorough elaboration.



   4.1. 1           Introduc tio n
   The BSCAT method refers to a method that links modern risk–based safety management approaches
   to systematic cause analysis (SCA) incident investigation. The “B” refers to barrier–based as each
   barrier identified in an event scenario is assessed on why it failed.

   SCAT stands for Systematic Cause Analysis Technique, a well-established root cause analysis
   approach which incorporates the DNV-GL loss causation model. The model is a sequence of dominos
   establishing the hierarchy of accident progression from the immediate cause back to fundamental
   root causes and system failures.

   In short - BSCAT is the barrier based extension to DNV-GL’s SCAT method.

   The SCAT model was developed to help incident investigators apply the DNV-GL loss causation model
   to actual events. This is done by means of the SCAT chart. The chart was created to build–out an
   event using standardized event descriptions that can fit a whole range of incidents and near misses.
   Due to using a standardized list, incident analyses are suitable for data aggregation, leading to more
   insight into the weak areas of your safety management system.

   A barrier-based accident investigation still applies the SCAT model but now it is applied to each
   barrier separately, not to the incident as a whole.

   The BSCAT chart is the latest update of the SCAT approach. It includes all the historical occupational
   safety topics and underlying causes, and moreover has added process risks to the list (e.g. fires and
   explosions) and is in general more suitable for barrier analysis.




© WOLTER S KLUWER | CG E R isk 2021              INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 26
                                                        MANUAL
                               Figure 37 - Difference between SCAT and BSCAT


   4.1. 2          BSCAT work proce ss
   The figure below outlines the main steps in a BSCAT accident investigation.




                              Figure 38 - BSCAT accident investigation overview


   The BSCAT phase consists of the following steps.

       1. Evidence is collected and preserved.

   Evidence collection is done offline (not in the software).




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                  PAGE 27
                                                    MANUAL
       2. A timeline/storyboard of events is built. At its simplest, a timeline is a listing of all relevant
          events or factors in a time-based sequence. Simple incidents may not require a timeline, but
          using one often helps understand complex incidents involving many people, different
          systems, and how latent issues (such as design or construction deficiencies or unrevealed
          failures) affect an outcome.

   As already seen, the software supports building the timeline in a user-friendly manner. See section
   3.4, Creating a timeline.

       3. After creating the timeline, key events are identified and placed in an event flow diagram.

   The software supports building these diagrams in two manners – in the completely free form, but
   also using pro-active bowtie risk analysis as a source for event flow information – this is known as
   building a templated BSCAT diagram.

   For free-form building of a BSCAT diagram, see section 4.2, Creating a BSCAT diagram.
   For templated building of a BSCAT diagram, see section 10, Linking incidents with bowtie.

       4. Barriers thereafter are identified and inserted between the key events (only in the free-form
          building).
       5. Barriers are then analyzed; Which have failed? Which were missing? Etc.
       6. A BSCAT analysis is performed for each (failing) barrier.

   Steps two to five are repeated for each hypothesis about the accident causation.

       7. In the last step, a report is generated.

   The BSCAT tool supports many different reports and output methods to provide you with all
   information needed to create a full report. See section 14, Getting data out of the program.

   Optionally, the barriers identified in the investigation can be linked to barriers on pro-active bowtie
   risk analyses to provide aggregation and insight into a scenario based context. If applicable bowties
   do not (yet) exist, they can be built (given you have a BSCAT IncidentXP + BowTieXP license). For
   linking incidents with bowtie risk analysis, see section 10, Linking incidents with bowtie.


   4.2      CREATING A BSCAT D IAGRAM
   The BSCAT diagram consists of (a chain of) events, that are linked through connectors. To start a
   diagram, we must start with a single event.

   When selecting the BSCAT method in creating a new incident (see section 3.2, Creating an incident &
   incident analysis diagram), the software automatically provides the first event of a BSCAT diagram, so
   you can get started on the BSCAT diagram immediately.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                     PAGE 28
                                                    MANUAL
                                      Figure 39 - Default start of a BSCAT diagram


   To create a new BSCAT diagram, you have to right-click the incident node and select ‘New Incident’.
   Select the BSCAT methodology in the add incident diagram dialog and give the diagram a name.




                                       Figure 40 – Creating a new BSCAT diagram


   4.3       ADD A BSCAT EVENT


       Note: It is also possible to add BSCAT events by selecting events (top event, threats, or consequences) from a bowtie
       diagram. In section 10, Linking incidents with bowtie you can read how to do this.


   To add a new BSCAT event hover your mouse over the right or left side of an existing BSCAT event
   and click the green button that appears. The left button will add an incoming event (a previous event
   leading to this one). The right button will add an outgoing event (an event which happened after this
   one).




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                       PAGE 29
                                                            MANUAL
                                     Figure 41 - Add a new BSCAT event


   It is also possible to add a new BSCAT event by clicking this button in the toolbar:

   You can edit an event by right clicking an existing event and choose Edit event. Other option are to
   double click the element or use the pencil icon. Give the BSCAT event a name and decide about the
   appearance of the event – it can be shown as a normal event, a threat, a consequence, a top event as
   well as a potential event (one which did not happen, but could have taken place).




                                        Figure 42 - BSCAT event editor


   These are the different appearance options for the events:




   These appearance options are to differentiate the types of events. The first event (red) is the default
   BSCAT event appearance. The next three events are mainly used in the bowtie theory. The last is an
   exceptional element to specifically stand out.

   The event is added to the diagram after clicking OK:




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                   PAGE 30
                                                    MANUAL
                                              Figure 43 - Added BSCAT event


   We can reposition the events by dragging them around the diagram. A background grid is shown to
   assist you in spacing your diagram. Shapes will automatically snap to the grid when dropping them.
   This is called the assisted layout and is used by BSCAT diagrams by default. There is also a free grid
   layout. This means that the diagram shapes can freely be distributed across the diagram space. To
   change the layout grid, please see section 11.4 Layout grid.


   4.3. 1            Add a BSCAT event between two events
   To add an event in between the two events, we must first create some space in case the events are
   placed in close proximity of each other. To achieve this, drag the rightmost event one (or more)
   slot(s) to the right.




                                       Figure 44 - Creating space between events


   When we move the mouse over the connector, we see two buttons appearing. When clicking the
   blue icon, an event is automatically created in between the other events.




                                   Figure 45 - Add an event between of two events


       Note: It is also possible to add BSCAT events by selecting events (top event, threats, or consequences) from a bowtie
       diagram. In section 10, Linking incidents with bowtie you can read how to do this



   4.3. 2            Add a BSCAT e ve nt in paralle l to anothe r e ve nt
   When one event results into multiple events, we need to add different events in parallel. Move the
   mouse over the right side of the first (outmost left) event, and a green button will appear.




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                       PAGE 31
                                                            MANUAL
                            Figure 46 - Add an event in parallel to another event


   When we click this button, we can add an event parallel to the other event.




                                      Figure 47 - A parallel BSCAT event


   The same can be done if multiple initial events result in one following event.


   4.4      ADD A BSCAT INCIDENT BARRIER
   To add an incident barrier in between two events, we again must create some space. Drag the
   rightmost event one (or more) slot(s) to the right, such as in Figure 44. When we move the mouse
   over the connector, we again see two buttons appearing. When clicking the grey icon, a barrier is
   automatically created on the connector between the two events.




                                      Figure 48 - Add an incident barrier


   Besides giving the barrier a title, we can enter a description, barrier type, enter how the barrier
   failed, both in a short description and in a longer multiline format, as well as choose the barrier state
   – also known as the failure state: failed, missing, unreliable, inadequate or effective. We can enter
   the barrier challenge and response in the ‘Barrier Challenge/Barrier Response’ tab. Also we can add
   information in a tab called ‘User Data’ where we can define our own fields, if so required. Please see
   the BowTieXP software manual for more information regarding this tab. More information regarding
   the ‘Document Links’ tab is available in chapter 13
   Linking to documentation.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 32
                                                    MANUAL
                                     Figure 49 - Incident barrier editor


   The incident barrier states are drawn on the diagram as follows:




                                     Figure 50 - Incident barrier states


   Using the skills described above we can quickly build more complex diagrams:




                                 Figure 51 - Example BSCAT diagram (1/2)


   Adding additional barrier information results in an even more elaborate picture:




                                 Figure 52 - Example BSCAT diagram (2/2)



© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                      PAGE 33
                                                   MANUAL
   4.5      AD D A BSCAT CAUSATIO N ASSESSMEN T
   Once the diagram has been completed, we need to assess all its barriers using the BSCAT method.

   To add a new BSCAT causation path you can move your mouse over the bottom of an existing BSCAT
   incident barrier and click the light red button that appears.




                                   Figure 53 - Add new BSCAT causation


   The BSCAT causation editor will allow you to do three things:
       • Add a description of the immediate cause, basic cause and management system factor.
       • Assign BSCAT chart values to the immediate cause, basic cause and management system
          factor.
       • Add document links or user data to the barrier.

   Some users like to add both the descriptions and assign the BSCAT chart values as well, other users
   choose only one of these two options.

   The editor tab in the BSCAT causation editor window allows you to add a description of the
   immediate cause, basic cause and management system factor, and you are also able to indicate what
   particular kind of inadequacy of the management system lead to this barrier failure (toggle the
   appropriate option).




                Figure 54 - Add descriptions and inadequacy in the BSCAT causation editor




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 34
                                                   MANUAL
   The BSCAT Immediate causes, BSCAT Basic causes and BSCAT Management System Factors tabs in
   the BSCAT causation editor allow you to assign the BSCAT chart values. Select one of these tabs and
   move the applicable BSCAT chart value from the left to the right window, using the blue arrows or by
   dragging and dropping the values:




                                 Figure 55 - Link BSCAT chart to the BSCAT causation


       Note #1: If you want to add a second BSCAT causation to the same incident barrier, right click the incident barrier
       and choose Add → BSCAT causation.


       Note #2: The Expand/Collapse buttons and the Filter are useful features in the edit window to quickly find the chart
       values you are looking for.


       Note #3: The BSCAT chart values can also be assigned to an incident barrier by dragging and dropping them from
       the treeview.



   4.5. 1            BSCAT chart in the treeview
   The entire BSCAT chart with all three level cause categorizations is included in the case file. You ca n
   find them defaulted in the treeview, within the incident lookup tables:




© WOLTER S KLUWER | CG E R isk 2021                 INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                      PAGE 35
                                                           MANUAL
                     Figure 56 - BSCAT chart in the treeview (collapsed and expanded)


   Expand the ‘parent’ to be able to see all the ‘children’ within one BSCAT category.



   4.6      TH E CO NNECTO R
   The incident connector is the line which connects the incidents’ events. To create a new connector,
   right click on some empty space and select Add, Connector, and then select the events you want to
   connect. Alternatively, to add a new connector, move your mouse over the right or left side of an
   existing event and hold on the green button that appears. While holding, drag the line to another
   event to connect both events.




                              Figure 57 - Add new connector (two alternatives)


   4.7      ED IT TH E BSCAT D IAGRAM
   We will list methods of editing the diagram here in order to create a complete overview:
      • To edit a shape, double click it or select it and press F2.
      • To change to order of barriers, drag them left or right beyond its neighbor.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                PAGE 36
                                                   MANUAL
       •    It is possible to copy (CTRL+C) and paste (CTRL+V) shapes with the clipboard (it is also
            possible to copy paste shapes from the timeline to an analysis diagram).
       •    Hold down SHIFT to also move connecting shapes.
       •    To copy additional properties, use paste special. This allows you to select in a separate pop
            up screen which items to paste over the selected element.
       •    To remove an item, select it and press DEL.
       •    To move barriers, select it, right-click it and select cut. Click the destination and select paste.


       Note: The layout engine in the BSCAT software has been optimized for BSCAT diagrams – diagrams with a limited
       number of parallel paths. If you attempt to create for instance a fault tree or event tree type diagrams, you will
       quickly run into the limitations of the automatic layout function.

       Breaking the move of items you want to accomplish into separate steps will alleviate this problem. With separate
       steps we mean more than one drag and drop operation. Instead of trying to move an item immediately to the
       wanted end position, do it in smaller steps. E.g. first move it left, and then move it up. This also helps you understand
       what the layout engine is trying to do and make the system work for you, instead of against you.




   5
   THE TRIPOD BETA METHOD
   5.1  A BRIEF D ESCRIPTIO N O F TRIPO D BETA
   METH O D O LO GY


       Note: Please see Stichting Tripod Foundation’s Tripod Beta User Guide for a more thorough elaboration.



   5.1. 1             Introduc tio n
   Developed in the early 1990s, Tripod Beta is an incident investigation methodology designed in line
   with the human behavior model. It was explicitly created to help accident investigators model
   incidents in a way that allows them to understand the influencing environment, and to uncover the
   root organizational deficiencies that allowed an incident to happen.

   Using Tripod Beta, incident investigators model incidents in terms of:
       • Objects (something acted upon, such as a flammable substance or a piece of equipment),
       • Agent of change (something –often an energy- that acts upon objects, such as a person or a
           fire) and
       • Events (the result of an agent acting upon an object, such as an explosion).

   Working back from the 'main event' (the incident) allows us to fully understand what happened and
   how an incident happened.

   A set of shapes consisting of an agent, an object and an event is called a trio and is the basic building
   block of the Tripod beta method. Events themselves can also be objects or agents, allowing us to
   chain these trios into a contextual diagram.




© WOLTER S KLUWER | CG E R isk 2021                   INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                          PAGE 37
                                                             MANUAL
   To understand why the incident happened, the next step is to determine what barriers were in place
   that should have prevented the objects’ and agents’ outcome, and also why these barriers failed to
   do so. Tripod Beta teaches us to look at the immediate causes of the act that led to the incident, the
   psychological precursors to that immediate cause, and ultimately the underlying organizational
   deficiencies that allowed those precursors to exist.




                                       Figure 58 - A sample Tripod Beta diagram1


   5.1. 2             Tripod Be ta analysis work proce ss
   Doing Tripod Beta analysis consists of the following steps.

       1. Evidence is collected and preserved.

   Evidence collection is done offline (not in the software).

       2. A timeline/storyboard of events is built. At its simplest, a timeline is a listing of all relevant
          events or factors in a time-based sequence. Simple incidents may not require a timeline, but
          this tool often helps understand complex incidents involving many people, different systems,
          and how latent issues (such as design or construction deficiencies or unrevealed failures)
          affect the outcome.

   As already seen, the software supports building the timeline in a user-friendly manner. See section
   3.4, Creating a timeline.

       3. After creating the timeline, the trio’s (agent, object, and event) are identified and linked to
          each other which forms an event flow diagram. Linking an event to an agent results in a
          combined node called ‘event-agent’. Linking an event to an object results in a combined
          node called ‘event-object’.

   The software supports building these diagrams in two manners – in the completely free form, but
   also using pro-active bowtie risk analysis as a source for event flow information – this is known as
   building a templated Tripod Beta diagram.




   1 Source: Stichting Tripod Foundation.




© WOLTER S KLUWER | CG E R isk 2021              INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 38
                                                        MANUAL
   For free-form building of a Tripod Beta diagram, see section 5.2, Creating a Tripod Beta diagram and
   section 5.3 Add a Tripod trio (Event – Agent – Object). For templated building of a Tripod Beta
   diagram, see section 10, Linking incidents with bowtie.

       4. Barriers are identified and inserted between the event and agent or object.

   See section 5.4 Add a Tripod Beta incident barrier.

       5. Barriers are analyzed; Which have failed? How did they fail? Which were missing?.

   Please see Stichting Tripod Foundation’s Tripod Beta User Guide for a thorough elaboration on this.

       6. A Tripod Beta causation assessment is done on each barrier.

   See section 5.5, Add a Tripod Beta causation assessment.

       7. Finally, a report is generated.

   The Tripod Beta tool supports many different reports and output methods to provide you with all
   information needed to create a full report. See chapter 14, Getting data out of the program.

   Optionally, the barriers identified in the investigation can be linked to barriers on pro-active bowtie
   risk analyses to provide aggregation and insight into how barriers relate to real life events. If
   applicable bowties do not (yet) exist they can be created (given that you have a Tripod IncidentXP +
   BowTieXP license). For a templated building of a Tripod Beta diagram, see section 10, Linking
   incidents with bowtie.



   5.2       CREATING A TRIPO D BETA D IAGRAM


       Note: The Tripod Beta diagrams shown in this manual may slightly deviate from the Tripod Beta rules which are set
       by the Stichting Tripod Foundation. These differences may especially occur within the description of barriers. This
       manual will often use the bowtie risk assessment guidelines for describing barriers.


   The Tripod diagram consists of events, agents and objects, which are connected through connectors.
   To start a diagram, we must start with a single event.

   To create a new Tripod Beta diagram, right-click the incident node in the treeview and select ‘New
   Incident’. Name the Tripod diagram and select methodology ‘Tripod Beta’.




© WOLTER S KLUWER | CG E R isk 2021                 INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                     PAGE 39
                                                           MANUAL
                                Figure 59 – Creating a new Tripod diagram


   When the method ‘Tripod Beta’ is selected whenever an incident is created (see section 0, Creating
   an incident & incident analysis diagram), the software automatically creates the first event of a
   Tripod diagram, so you can get started on the Tripod diagram immediately.




                             Figure 60 - Default start of a Tripod Beta diagram


   5.3      AD D A TRIPO D TRIO (EV ENT – AGENT – O BJECT)
   To add a new Tripod trio, move your mouse over the left side of an existing Tripod event and click the
   button that appears. The left button will allow you to add a tripod trio (new Agent & Object). The
   right button will add an outgoing event (an event which happened after the initial event).




                                     Figure 61 - Add a new Tripod trio




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 40
                                                   MANUAL
                                        Figure 62- Added Tripod trio


   Double click the outlines of the Agent and Object to open the editor and edit the elements’ name
   and appearance:




                                       Figure 63 - Tripod event editor


   This dialog box also allows you to change the appearance of the element – which can be shown as
   Agent, Object, Event, Event-Agent, Event-Object as well as a Potential event (an event that did not
   happen, but could have occurred). You can also convert shapes into bowtie appearances: Threat,
   Consequence, and Top event. Please refer to chapter 15 for details on this.




                          Figure 65 - All possible Tripod beta appearance options



© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 41
                                                   MANUAL
   Creating a Tripod-Beta incident analysis diagram is an iterative process. Because of this, the software
   allows you to reposition the events by dragging them around the diagram. A grid will be shown while
   doing so. By default, Tripod-Beta diagrams use a free layout grid. The free layout allows the user to
   distribute the diagram shapes freely on the diagram space by simply moving them or using cutting
   and pasting movements. In this way, Tripod diagrams can easily be rearranged.


   5.3. 1            Add a Tripod O bje ct in paralle l to an e x isting
   trio

       Note: this sub-chapter describes IncidentXP functionality that is not part of the official Tripod Beta guidelines that
       are issued by the Energy Institute. In the official guidelines, there can be only one object against an agent.


   If you want to add a second object to an existing Tripod trio, move the mouse over the left side of
   the event, and a green button will appear. When we click this button, a second object will
   automatically be added. Alternatively, you can right-click on an event and select “Add Event to the
   left”. The editor pops up and we can enter the details of the object and change the appearance, for
   instance to Object or Agent.




                                 Figure 66 - Add an event in parallel to another event


       Note: the newly added element(s) might be partially placed over existing ele ment(s). Use the free layout to drag
       and drop the elements to the preferred position.


       Note: It is also possible to add Tripod elements by selecting events (top event, threats, or consequences) from a
       bowtie diagram. In section 10, Linking incidents with bowtie you can read how to do this.



   5.4       AD D A TRIPO D BETA INCID ENT BARRIER
   To add a barrier in between two elements, we must first create some space when the shapes are
   placed in close proximity of each other. To create space between elements, drag the rightmost shape
   to the right.




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                        PAGE 42
                                                            MANUAL
                                Figure 64 - Creating space between elements


   When we hover with the mouse over a connector, we can see both a grey barrier button and a blue
   event button. If we click the grey barrier button, we can add a barrier to the connector.




                                     Figure 65 - Add an incident barrier


   Right click on a barrier and click Edit Incident Barrier. In this editor box, we can give the barrier a
   name, enter a description, choose a barrier type, enter how the barrier failed, both in a short
   description and in a longer multiline format, as well as choose a barrier status – also known as the
   failure state: failed, missing, unreliable, inadequate (or effective when not failed). We can provide
   the barrier challenge and response in the ‘Barrier Challenge/Barrier Response’ tab. Additionally, we
   can add information in a tab called ‘User Data’ where we can define our own fields, if so required.
   Please see the BowTieXP software manual for more information regarding the ‘User Data’ tab. More
   information regarding the ‘Document Links’ tab is available in chapter 13
   Linking to documentation.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                   PAGE 43
                                                    MANUAL
                                             Figure 66 - Incident barrier editor


   The incident barrier states are shown on the diagram as follows:




                                             Figure 67 - Incident barrier states


   Using these software options allows you to quickly build complex diagrams such as:




                  Figure 68 - Example of Tripod Beta diagram (without causation assessment)


       Note: The ‘Unreliable’ barrier state is not part of the official Tripod Beta guidelines that are issued by the Energy
       Institute. In the official guidelines, this state does not exist. It is however part of IncidentXP for it is used within
       other methods that IncidentXP supports (i.e. BFA and BSCAT)




© WOLTER S KLUWER | CG E R isk 2021                   INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                          PAGE 44
                                                             MANUAL
   5.5      AD D A TRIPO D BETA CAUSATIO N ASSESSMEN T
   Once you complete the diagram it is time to assess all the barriers using the Tripod Beta method. As
   mentioned earlier, Tripod Beta uses three levels of causation: Immediate causes, Preconditions, and
   Underlying causes.

   To add a new Tripod causation path, move your mouse over the bottom of an existing incident
   barrier and click the grey button that appears.




                             Figure 69 - Add new Tripod causation assessment


   Right click on the Immediate Cause that appeared in the diagram, and select Edit Tripod Immediate
   Cause. In the Immediate Cause editor window we can fill in the details of the immediate cause.




                     Figure 70 - Add descriptions in the Tripod Immediate Cause editor


   When we click OK the immediate cause is added to the barrier. You can drag the immediate cause to
   position its place above or below the barrier:




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                PAGE 45
                                                   MANUAL
                                Figure 71 - Added Tripod Immediate Cause


   You can add one or more precondition(s) and underlying cause(s) by following the same steps. This
   time move the mouse over the left side of the immediate cause and precondition to let a blue and
   yellow button appear instead.




                        Figure 72 - Add a new Precondition and Underlying Cause


   The Tripod Beta causation path is completed after adding the underlying cause.




                           Figure 73 - Completed Tripod causation assessment




© WOLTER S KLUWER | CG E R isk 2021        INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                              PAGE 46
                                                  MANUAL
       Note #1: Officially missing barriers can only have underlying causes linked to it. However, this rule has recently been
       changed (allowing also immediate cause and preconditions on missing barriers). IncidentXP supp orts the original
       rule and the revised rule.

       Note #2: IncidentXP supports categorizing your immediate causes, preconditions, and underlying causes. The list of
       categories can be defined within the Incident Lookup Tables node of your case file.



   5.5. 1             Add a Basic Risk F actor (BRF )
   In the Underlying Cause editor, you can define a basic risk factor (BRF) code. This categorization can
   be used to perform a trend analysis on your underlying causes. Aggregating BRF codes can provide
   valuable insights, for instance to highlight weaknesses within the organization for multiple incidents
   and over longer periods of time. BRF codes can be customized in the tree view.




                                                  Figure 74 - Add BRF code


   A BRF code can be visualized on an underlying cause by using the small eye icon on the causation
   element:




                                           Figure 75 - Visualization of BRF code


   5.6        TH E CO NNECTO R
   The incident connector is the line which connects the incidents’ events. To create a new connector,
   right click on some empty space and select Add, Connector, and then select the events you want to
   connect. Alternatively, to add a new connector, move your mouse over the right or left side of an
   existing event and hold on the green button that appears. While holding, drag the line to another
   event to connect both events.



© WOLTER S KLUWER | CG E R isk 2021                   INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                         PAGE 47
                                                             MANUAL
                              Figure 76 - Add new connector (two alternatives)


   5.7      ED ITING YO UR TRIPO D BETA D IAGRAM
   Here is a list of all methods for editing the diagram shapes:
      • To edit a shape, double click it or select it and press F2.
      • To change the order of barriers, drag them left or right beyond its neighbor.
      • Hold down SHIFT to also move connecting shapes.
      • It is possible to copy (CTRL+C) and paste (CTRL+V) shapes with the clipboard.
      • To copy some properties, Use paste special. This allows you to select which items to paste
           over the selection.
      • To remove an item, select it and press DEL.




   6
   THE BARRIER FAILURE ANALYSIS (BFA)
   METHOD
   6.1      A BRIEF D ESCRIPTIO N O F BF A METH O D O LO GY
   Barrier Failure Analysis (BFA) is a pragmatic, un-opinionated, general-purpose incident analysis
   method. It has no affiliation with any particular bureau, authority or organization.

   BFA is a way to structure incidents and to categorize certain parts of incident taxonomy. The
   structure consists of events, barriers and causation paths. Events are used to describe a causal
   sequence of events that we want to prevent from cascading. This means that each event causes the
   occurrence of a next event. There can also be parallel events that combined cause the next event to
   happen.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 48
                                                   MANUAL
   The barriers in a BFA are designed to stop a chain of events. They are not necessarily independent or
   sufficient, as would be the case in official bowtie guidelines. Since a barrier failure is presumed in this
   technique, causation paths are determined to explain why the barriers did not perform according to
   function.

   The causation consists of three steps, which are respectively called Primary, Secondary and Tertiary
   level. These labels can be customized. The method does not specify whether the analysis should end
   on an organizational level or not, this is merely what the software allows you to do. Each level in the
   causation path can additionally be categorized to your own fit. By default there are no categories
   defined in the software. This allows you as a user the flexibility to create relevant, custom categories
   specific to your own organizational context.

   Any organization should go through an initial period of testing and iterating these categories. At
   some point a steady state should emerge that will capture most incidents and provide relevant
   insights. Whenever diversions from this structure were to occur more often, it is possible to keep this
   learning process alive and reconsider the initial categorization.


   6.2       CREATING A BF A D IAGRAM
   The BFA diagram consists of events, which are connected through connectors. To start a diagram we
   must begin with a single event.

   If the method BFA is selected when creating an incident (section 3.2, Creating an incident & incident
   analysis diagram), the software automatically creates the first event of a BFA diagram, in order for
   you to get started on the BFA diagram immediately.




                                  Figure 77 - Default start of a BFA diagram


   To create a new BFA diagram, you have to right-click the incident node in the treeview and select
   ‘New Incident’. Name the diagram and select the ‘BFA’ as your analysis methodology.




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                      PAGE 49
                                                     MANUAL
                                      Figure 78 – Creating a new BFA diagram


   6.3       ADD A BFA EVENT


       Note: It is also possible to add BFA events by selecting events from a bowtie diagram (top event, threats, or
       consequences). In section 10, Linking incidents with bowtie you can read how to do this.


   To add a new BFA event move your mouse over the right or left side of an existing BFA event and
   click the green button that appears. The left button will add an incoming event (a previous event
   leading to this one). The right button will add an outgoing event (an event which happened after this
   one). Alternatively, you can right-click on an event and select “Add - Event to the left (or right)”.




                                          Figure 79 - Add a new BFA event


   It is also possible to add a new BFA event by clicking this button in the toolbar:

   Right-click on an existing event and choose Edit Event to give the BFA event its name and appearance
   – it can be altered into an event, a threat, a consequence, a top event as well as a potential event
   (something that did not happen, but could have happened).




© WOLTER S KLUWER | CG E R isk 2021               INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                               PAGE 50
                                                         MANUAL
                                        Figure 80 - BFA event editor


   These are the different appearance options for BFA events:




   These appearance options are to differentiate what kind of event we are considering. The first
   appearance (event) is the default BFA event appearance. The appearances threat, consequence and
   top event refer to the bowtie theory. Potential but not reached is an addition to highlight possible
   outcomes that did not happen in the incident.

   When you click OK an event is added to the diagram:




                                        Figure 81 - Added BFA event


   We can reposition the events by dragging them around the diagram. A background grid is shown to
   assist you in spacing your diagram. Shapes will automatically snap to the grid when dropping them.
   This is called the assisted layout and is used by BFA diagrams by default. There is also a free grid
   layout. This means that the diagram shapes can freely be distributed across the diagram space. To
   change the layout grid, please see section 11.4 Layout grid.


   6.3. 1          Add a BF A event between two events
   To add an event in between the two events, we must first create some space in case the events are
   placed in close proximity of each other. To achieve this, drag the rightmost event one (or more)
   slot(s) to the right.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 51
                                                   MANUAL
                                     Figure 82 - Creating space between events


   When we move the mouse over the connector, we see two buttons appearing. When clicking the
   blue icon, an event is automatically created in between the other events.




                                    Figure 83 - Add an event between two events


   6.3. 2           Add a BF A e ve nt in paralle l to anothe r e ve nt
   When one event results into multiple events, we need to add different events in parallel. Move the
   mouse over the right side of the first (outmost left) event, and a green button will appear. When we
   click this green button, we can add an event parallel to the initial event. The editor pops up after
   double clicking the event. You can now again enter the details of this event.




                             Figure 84 - Add an event in parallel to another event


   The result can look like this:




                                          Figure 85 - A parallel BFA event


   A similar exercise can be performed the other way around when multiple events result in one event.




© WOLTER S KLUWER | CG E R isk 2021             INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                PAGE 52
                                                       MANUAL
   6.4      ADD A BFA INCIDENT BARRIER
   To add an incident barrier in between two events, we again must create some space. Drag the
   rightmost event one (or more) slot(s) to the right, such as in Figure 44. When we move the mouse
   over the connector, we again see two buttons appearing. When clicking the grey icon, a barrier is
   automatically created on the connector between the two events.




                                      Figure 86 - Add an incident barrier


   Besides giving the barrier a title, we can enter a description, barrier type, enter how the barrier
   failed, both in a short description and in a longer multiline format, as well as choose the barrier state
   – also known as the failure state: failed, missing, unreliable, inadequate or effective. We can enter
   the barrier challenge and response in the ‘Barrier Challenge/Barrier Response’ tab. Besides that you
   can add information in a tab called ‘User Data’ where we can define our own fields, if so required.
   Please see the BowTieXP software manual for more information regarding this tab. More information
   regarding the ‘Document Links’ tab is available in chapter 13
   Linking to documentation.




                                      Figure 87 - Incident barrier editor


   The incident barrier states are drawn on the diagram as follows:




                                      Figure 88 - Incident barrier states




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 53
                                                    MANUAL
   Using these building techniques, we can quickly constitute complex diagrams such as these:




                                         Figure 89 - Example BFA diagram (1/2)




                                         Figure 90 - Example BFA diagram (2/2)


   6.5       AD D A BF A CAUSATIO N ASSESSMEN T
   Once the diagram has been fully constructed, we need to assess all its barriers using the three step
   causation method.

   To add a new BFA causation move your mouse over the bottom of an existing incident barrier and
   click the grey button that appears.




                                            Figure 91 - Add new BFA causation


   The BFA causation editor will allow you to do two things:
       • Add a description of the three step causation - why the barrier failed/was
          missing/unreliable/inadequate.
       • Link your company incident causation chart values to the description of the three-step
          causation.


       Note: How to import your own company’s incident causation chart is described in section 6.5.1
       Incident causation chart. By default, this list will be empty for the BFA categories within the software.




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                   PAGE 54
                                                            MANUAL
   Some users like to add both the flexible description and link the incident causation chart too, while
   other users choose one of these two options. The editor tab in the BFA causation editor allows you to
   add a description of the primary cause, secondary cause and tertiary cause.




                              Figure 92 - Add descriptions in the BFA causation editor


   Accordingly, custom incident causation charts can be found under the BFA Primary causes, BFA
   Secondary causes and BFA Tertiary causes tabs in the BFA causation editor. These tabs allow you to
   link the BFA chart values to the causation descriptions. Select one of these tabs and move the
   applicable chart value from the left to the right window, using either the appointed arrows or by
   dragging and dropping the value. Defaulted in the software, the causation charts in the tree view are
   empty.




             Figure 93 - Incident causation chart value linked to the BFA causation (now empty)


       Note: If you want to add a second incident causation path to the same incident barrier, right -click the incident
       barrier and choose Add → BFA causation.


   6.5. 1            Incident causatio n chart


© WOLTER S KLUWER | CG E R isk 2021                INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                   PAGE 55
                                                          MANUAL
   It is possible to create a customized incident causation chart with all three level cause categorizations
   within the tree view of your case file. If you expand the Incident lookup tables node, you will find the
   BFA causation chart nodes. As earlier mentioned, these nodes are defaulted empty in the software.
   An example of customized BFA causation categories is shown below:




                                Figure 94 – Example incident causation chart


   How to adjust the lookup tables can be found in section 11.3, Adjusting lookup tables.


   6.6      TH E CO NNECTO R
   The incident connector is the line which connects the incidents’ events. To create a new connector,
   right click on some empty space and select Add, Connector, and then select the events you want to
   connect. Alternatively, to add a new connector, move your mouse over the right or left side of an
   existing event and hold on the green button that appears. While holding, drag the line to another
   event to connect both events.




                             Figure 95 - Add a new connector (two alternatives)


   6.7      ED IT TH E BF A D IAGRAM
   Here is a list of all methods for editing the diagram shapes:
      • To edit a shape, double click it or select it and press F2.
      • To change the order of barriers, drag them left or right beyond its neighbour.
      • Hold down SHIFT to also move connecting shapes.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 56
                                                    MANUAL
      •    It is possible to copy (CTRL+C) and paste (CTRL+V) shapes with the clipboard.
      •    To copy some properties, Use paste special. This allows you to select which items to paste
           over the selection.
      •    To remove an item, select it and press DEL.
      •    To move barriers, select it, right-click it and select cut. Click the destination and select paste.


      Note: The layout engine in the IncidentXP software has been optimized for BFA diagrams – diagrams with not too
      many parallel paths. If you attempt to create very complex diagrams, you could run into the limitations of the
      automatic layout function.

      Breaking the movement of items into separate steps will alleviate this problem. With separate steps we mean more
      than one drag and drop operation. Instead of trying to move an item immediately to the wanted end position, do
      it in smaller steps. E.g. first move it left, and then move it up. This also helps you understand what the layout engine
      is trying to do and make the system work for you, instead of against you.




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                         PAGE 57
                                                            MANUAL
   7
   THE ROOT CAUSE ANALYSIS (RCA)
   METHOD
   7.1      A BRIEF D ESCRIPTIO N O F RCA METH O D O LO GY
   Root Cause Analysis (RCA) is a simple and straightforward incident analysis technique. It starts off
   with an incident and drills down into the chain of events that led to that incident until the root
   causes are identified. This method is widely used throughout the world, and the idea of drilling down
   to the root cause is also present in all of our other incident analysis methods.

   As a downside, a traditional root cause analysis has the potential to turn into a jumble of elements.
   We felt this could be improved, so we did two things in our software solution. Firstly, we added some
   categorizations to see at a glance where the core problem areas accumulate. This enables you to
   perform a concise analysis that is easier to communicate. Secondly, you can separate the main
   diagram into smaller sub-parts. This avoids you losing overview. Expectedly these changes to RCA will
   help you create better analyses.

   7.1. 1         7.1.1. Barriers vs RCA
   The main difference between RCA and our other incident analysis methods, lies in the fact that RCA is
   not barrier based. Everything in RCA is an event, including the elements that would be considered
   barriers or barrier failures in BSCAT, Tripod or BFA. However, from software version 9.2.0 onwards
   you are able to link RCA events to bowtie barriers. By doing so you can signal if an event can be seen
   as a failed barrier. To find out how to do this, please see chapter 10 Linking incidents with bowtie.


   7.2      CREATING AN RCA D IAGRAM
   To create a new RCA diagram, you have to right-click the incident node in the tree view and select
   ‘New Incident’. Name the diagram and select methodology ‘RCA’.




                                   Figure 96 - Creating new RCA diagram




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 58
                                                   MANUAL
   The RCA diagram consists of events, which are connected through connectors. To start building a
   diagram we must begin with a single event.

   When selecting the method RCA for incident analysis (see section 3.2, Creating an incident & incident
   analysis diagram on page 13), the software automatically creates the first event of a RCA diagram, so
   you can get started on the RCA diagram immediately.




                                 Figure 97 - Default start of an RCA diagram


   7.3      ADD AN RCA EVENT
   Add an event by clicking the green button on the left or right-hand side of the shape. An Event will
   automatically be created on the respective side.




                             Figure 98 - Adding an event using the green button


   Another way of adding an event is done by right-clicking the event and subsequently selecting Add →
   Add Event to the Left/Right.
   It is also possible to add a new RCA event by clicking this button in the toolbar:

   Using the pen icon in the bottom right of an event shape you can name the RCA event and decide
   about the appearance of the event. It can be shown as a 'Regular event’, a ‘Root cause’, a
   ‘Contributing cause’, ‘Confirmed not a cause’, ‘More info needed’, and a ‘Stop’ event.

   You can also assign a category and an equivalent Incident diagram. When an equivalent incident
   diagram is chosen, a link will be created from this particular event to another incident diagram. See
   section 7.6, for more information.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 59
                                                    MANUAL
                                        Figure 99 - RCA event editor


   Here are the different appearance options for the events:




   These appearance options are to differentiate what kind of event is at hand. The first event (regular)
   is the default RCA event appearance. When we click OK the event is added to the diagram:




                                       Figure 100 - Added RCA event


   We can reposition the events by dragging them around the diagram. By default, RCA diagrams use a
   free layout grid. The free layout allows the user to distribute the diagram shapes freely on the
   diagram space by simply moving them or using cutting and pasting movements. In this way, RCA
   diagrams can easily be rearranged. To change the layout grid, please see section 11.4 Layout grid.


   7.3. 1          Add an RCA e ve nt in paralle l to anothe r e ve nt
   To add events in parallel, move the mouse over the right side of the (first) event, and a green button
   will appear. When we click this green button, we can add an event parallel to the other event.
   Alternatively, you can right-click and select Add > Event to the right (or left).




                           Figure 101 - Add an event in parallel to another event



© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 60
                                                   MANUAL
   After repositioning the elements, the result can look like this:




                                       Figure 102 - A parallel RCA event


   7.4       TH E CO NNECTO R
   The incident connector is the line which connects the incidents’ events. To create a new connector,
   right click on some empty space and select Add, Connector, and then select the events you want to
   connect. Alternatively, to add a new connector, move your mouse over the right or left side of an
   existing event and hold on the green button that appears. While holding, drag the line to another
   event to connect both events.




                              Figure 103 - Add new connector (two alternatives)


   7.5       ED IT TH E RCA D IAGRAM
   Here is a list of all methods for editing the diagram shapes:
      • To edit a shape, double click it or select it and press F2.
      • To change the order of barriers, drag them left or right beyond its neighbor.
      • Hold down SHIFT to also move connecting shapes.
      • It is possible to copy (CTRL+C) and paste (CTRL+V) shapes with the clipboard.
      • To copy some properties, Use paste special. This allows you to select which items to paste
           over the selection.
      • To remove an item, select it and press DEL.




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                PAGE 61
                                                     MANUAL
   7.6       LINK RCA EV ENT TO EQUIV ALENT INCID ENT D IAGRAM
   When RCA diagrams become too large or when multiple incident investigators or investigation teams
   investigate different parts of the incident, a decision can be made to split the RCA diagram into
   different parts.

   Take the following steps to connect an RCA diagram to an RCA event from a different diagram within
   the same incident.
       1. Double click the RCA event which should be linked to the other RCA diagram.
       2. Choose the RCA diagram that should be linked to this event.
       3. Click OK.




                                   Figure 104 - Linking equivalent incident diagram


       Note: it is only possible to link to RCA diagrams within the same incident investigation (node).


   A blue arrow button will appear next to the event once a link has been created. You can click this
   arrow to navigate to the linked RCA diagram instantly.




                           Figure 105 - Navigate directly to equivalent incident diagram




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                          PAGE 62
                                                            MANUAL
   8
   KELVIN TOP-SET ROOT CAUSE ANALYSIS
   8.1       A BRIEF D ESCRIPTIO N O F RCA METH O D O LO GY
   Root Cause Analysis (RCA) is a simple and straightforward incident analysis technique. It starts off
   with an incident and drills down into the chain of events that led to that incident until the root
   causes are identified. This method is widely used throughout the world, and the idea of drilling down
   to the root cause is also present in all of our other incident analysis methods.

   As a downside, a traditional root cause analysis has the potential to turn into a jumble of elements.
   We felt this could be improved, so we did two things in our software solution. Firstly, we added some
   categorizations to see at a glance where the core problem areas accumulate. This enables you to
   perform a concise analysis that is easier to communicate. Secondly, you can separate the main
   diagram into smaller sub-parts. This avoids you losing overview. Expectedly these changes to RCA will
   help you create better analyses.

   8.1. 1            K e lvin TO P-SET RCA
   The TOP-SET® incident investigation methodology is used by major blue-chip clients all around the
   world. It is taught on 1-day to 3-day senior courses held throughout the world, and are the basis of
   several incident investigation products for professional incident investigators (Further details are
   available at www.kelvintopset.com).

   TOP-SET® was developed in 1989 to provide a comprehensive incident investigation methodology
   that would take users by the hand from the start of the process of investigation throughout the end.
   Users are educated in planning an investigation, visiting the scene and gathering data, interviewing
   witnesses, creating a storyboard of events through which incorporating as much information as
   possible, analysing the information to discover the causes, creating recommendations, and to report
   them.

   The Kelvin TOP-SET RCA method is a more structured approach than the regular RCA method. The
   TOP-SET acronym stands for Technology, Organization, People, Similar events, Environment and
   Time. It emphasizes on not to quit investigating too rapidly. We have all heard the phrase, “We all
   know what has happened here”, where the temptation is to make the RCA fit this early conclusion.
   Kelvin TOP-SET RCA encourages you to keep digging deeper until you arrive at the Root Cause of an
   incident.


       NOTE: Kelvin TOP-SET makes use of reference lists to ensure better quality incident investigations. Our software
       however, supports with the analysis part of incident investigation mostly. We advise you to contact Kelvin TOP -SET
       directly to gain a better understanding on their full methodology.



   8.1. 2            Barrie rs vs TO P-SET RCA




© WOLTER S KLUWER | CG E R isk 2021                 INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                     PAGE 63
                                                           MANUAL
   The main difference between TOP-SET RCA and our other incident analysis methods, lies in the fact
   that TOP-SET RCA is not barrier based. Everything in TOP-SET RCA is an event, including the elements
   that would be considered barriers or barrier failures in BSCAT, Tripod or BFA. However, from
   software version 9.2.0 onwards you are able to link TOP-SET RCA events to bowtie barriers. By doing
   so you can signal if an event can be seen as a failed barrier. To find out how to do this, please see
   chapter 10 Linking incidents with bowtie.


   8.2      CREATING A K ELV IN TO P -SET D IAGRAM
   To create a new TOP-SET RCA diagram, you have to right-click the incident node in the tree view and
   select ‘New Incident’. Name the diagram and select methodology ‘TOP-SET RCA’. It is possible to add
   additional incident information within this screen. After completion, press OK.




                              Figure 106 - Creating new TOP-SET RCA diagram


   The Kelvin TOP-SET RCA diagram consists of events, which are connected through connectors. To
   begin a diagram we start off from a single event.

   When selecting the TOP-SET RCA method for incident analysis (chapter 3.2 Creating an incident &
   incident analysis diagram), the software automatically creates the first event of a TOP-SET RCA
   diagram, enabling you to start the TOP-SET RCA diagram immediately.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                PAGE 64
                                                   MANUAL
                             Figure 107 - Default start of a TOP-SET RCA diagram


   Aside from automatically creating an event, it also shows the structure on which you can map your
   analysis.


   8.3      AD D AN TO P-SET RCA EV ENT
   Add an event by clicking the green button on the left, right, bottom or top side of the shape. An
   Event will automatically be created on the respective side.




                             Figure 108 - Adding an event using the green button


   Another way of adding an event is done by right-clicking the event and subsequently selecting Add →
   Add Event to the Left/Right.
   It is also possible to add a new RCA event by clicking this button in the toolbar:

   Using the pen icon in the bottom right of an event shape you can name the RCA event and decide
   about the appearance of the event. It can be shown as a 'Regular event’, a ‘Root cause’, a
   ‘Contributing cause’, ‘Confirmed not a cause’, ‘More info needed’, and a ‘Stop’ event.

   You can also assign a category and an equivalent Incident diagram. When an equivalent incident
   diagram is chosen, a link will be created from this particular event to another incident diagram. See
   section 7.67, for more information.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 65
                                                    MANUAL
                                   Figure 109 - TOP-SET RCA event editor


   Here are the different appearance options for the events:




   These appearance options are to differentiate the type of event. The first event (regular) is the
   default RCA event appearance. When we click OK the event is added to the diagram:




                                   Figure 110 - Added TOP-SET RCA event


   We can reposition the events by dragging them around the diagram. By default, RCA diagrams use a
   free layout grid. The free layout allows the user control over the layout of the diagram. This means
   that the diagram shapes can be freely distributed on the diagram space by simply moving them or
   with cutting and pasting movements. In this way, RCA diagrams can easily be rearranged. To change
   the layout grid, please see chapter 11.4 Layout grid.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                   PAGE 66
                                                    MANUAL
   8.4  AD D AN TO P-SET RCA EV ENT IN PARALLEL TO
   ANO TH ER EV ENT
   To add events in parallel, move the mouse over the right-and side of the (first) event, and a green
   button will appear. When we click this green button, we can add an event parallel to the other event.
   Alternatively, you can right-click and select Add > Event to the right (or left).




                             Figure 111 - Add an event in parallel to another event


   After dragging, the result can look like this:




                                   Figure 112 - A parallel TOP-SET RCA event


   8.5       TH E CO NNECTO R
   The incident connector is the line which connects the incidents’ events. To create a new connector,
   right click on some empty space and select Add, Connector, and then select the events you want to
   connect. Alternatively, to add a new connector, move your mouse over the right or left side of an
   existing event and hold on the green button that appears. While holding, drag the line to another
   event to connect both events.




                              Figure 113 - Add new connector (two alternatives)



© WOLTER S KLUWER | CG E R isk 2021            INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                PAGE 67
                                                      MANUAL
   8.6       ED IT TH E TO P-SET RCA D IAGRAM
   Here is a list of all the ways to edit the diagram shapes:
      • To edit a shape, double click it or select it and press F2.
      • To change the order of barriers, drag them left or right beyond its neighbor.
      • Hold down SHIFT to also move connecting shapes.
      • It is possible to copy (CTRL+C) and paste (CTRL+V) shapes with the clipboard.
      • To copy some properties, Use paste special. This allows you to select which items to paste
           over the selection.
      • To remove an item, select it and press DEL.


   8.7  LINK TO P-SET RCA EV ENT TO EQUIV ALEN T INCID ENT
   DIAGRAM
   When RCA diagrams become too large or when multiple incident investigators or investigation teams
   investigate different parts of the incident, a decision can be made to split the RCA diagram into
   different parts.

   Take the following steps to connect an RCA diagram to an RCA event from a different diagram within
   the same incident.
       1. Double click the RCA event which should be linked to the other RCA diagram.
       2. Choose the RCA diagram that should be linked to this event.
       3. Click OK.




                                   Figure 114 - Linking equivalent incident diagram

       Note: it is only possible to link to RCA diagrams within the same incident analysis (node).


   A blue arrow button will appear next to the event once a link has been created. You can click this
   arrow to navigate to the linked RCA diagram instantly.




© WOLTER S KLUWER | CG E R isk 2021                  INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                     PAGE 68
                                                            MANUAL
                         Figure 115 - Navigate to equivalent incident diagram




                      Figure 116 - An example of a finished TOP-SET RCA diagram




© WOLTER S KLUWER | CG E R isk 2021      INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                  PAGE 69
                                                MANUAL
   9
   ACTIONS / RECOMMENDATIONS
   Actions can be used in two ways, as TODO items during the incident investigation, or as
   recommendations (an improvement plan) as a result of the incident analysis. Actions are visualized
   on the analysis diagram as post-it shapes. The action code is also displayed in the sticky note shaped
   action.


   9.1      ACTIO NS AS A TO D O ITEMS
   The creation of incident diagrams has often already started during the initial information gathering
   phase of the incident investigation. Creating incident analysis diagrams gives the incident investigator
   insight into where information is lacking. Creating an action lets you track all the elements in your
   incident diagram that need further investigation. An example could look like this:




                    Figure 117 - Example of a TODO action during incident investigation


   9.2      ACTIO NS AS RECO MMEND A TI O N S
   After completing the incident analysis diagram, there is a better understanding of how and why the
   incident took place. Actions thereafter can also be used as recommendations to improve an
   organizations risk control framework and/or (safety) management system. Recommendations could
   refer to adding new barriers, improving existing barriers or changing the management system by
   focusing on an incidents root cause and fixing systemic failures. Action shapes can be used to
   highlight those areas.

   Some examples of actions as improvements are: correcting an error in a procedure, adding a new
   railing or changing the accountable party for a barrier. The improvements do not need to be
   recurring, but are implemented on a singular basis to improve the overall level of safety.

   Actions to create a new barrier are usually displayed on the associated event within the diagram,
   while (short-term) improvements on existing barriers are usually placed on the barrier itself.
   Improvements that cover deeper lying systemic causes (long-term) can be added to the causation
   path. An example could look like this:




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 70
                                                    MANUAL
                       Figure 118 - Example of recommendations on an incident diagram


   9.3       AD D ING AN ACTIO N
   To add an action, first select the element you want to add the action to. Adding an action can be
   done in two different ways:

       1. Clicking the action icon in the toolbar




                                       Figure 119 - Action button in the toolbar



       2. Right-clicking the element → Add → Action...


       Note: the action icon in the toolbar will only be activated once an element is selected, since actions need to be
       linked to a specific element of your diagram and can’t “float” in the diagram space.




© WOLTER S KLUWER | CG E R isk 2021                INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                   PAGE 71
                                                          MANUAL
                            Figure 120 - Add action by right-clicking an element


   The action editor will open and will allow you to add the following data:

       •   Code: the action code is displayed on the diagram.
       •   Name: shortly describes what the action is about.
       •   Description: a detailed description of the action.
       •   Category: a taxonomy for categorizing actions (NOTE: non-customizable).
       •   External Reference: possibility to fill in a reference to an external action tracking system.
       •   Action party: the job title responsible for completing this action (customizable in Incident
           Lookup Tables, as well as on the spot using the        icon).
       •   Organization unit: specific department or site of an organization (customizable in Incident
           Lookup Tables, defaulted as an empty node).
       •   Priority: gives a priority level to an action (customizable in Incident Lookup Tables).
       •   Target: sets a date before which the action has to be completed.
       •   Sign off comment: an optional field to provide comments when signing off an action.
       •   Sign off authorized by: the job title that authorized the completion (customizable in Incident
           Lookup Tables, as well as on the spot using the      icon).
       •   Sign off date: the date on which the action is signed off.
       •   Completed: this box can be toggled once the action has been completed.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 72
                                                    MANUAL
                                       Figure 121 - Add action window


   9.4      V ISUALIZING ACTIO NS
   Actions are visualized on the diagram as post-it shapes. The code is displayed within the shape. Go to
   Diagram → Display options or click the ‘large eye icon’ in the toolbar (    ) to adapt the visualization
   of actions.




                                 Figure 122a - Action shape display settings


   For Action Shapes, select either ‘None’ to hide all actions, choose ‘Overdue’ to show all uncompleted
   actions with an overdue target date, tick ‘Uncompleted’ to show only uncompleted actions or select
   ‘All’ to see every action shape.

   In the Colors tab you can choose to color your actions by Priority, Category, Overdue or Completed
   status.




                                  Figure 123 - Action color display settings



© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                   PAGE 73
                                                    MANUAL
   9.5       ACTIO N REPO RTS
   There are specific reports available for actions. Go to Tools → Reports or click the report button in

   the main toolbar (       ). Then expand the Actions section to see the following action reports:

       •   Actions in XL: a flat list of Actions, opens in Excel
       •   All Actions: a list of all actions in alphabetical order based on code, opens in Word
       •   Actions by Action party: a report with all actions sorted by Action party, opens in Word
       •   Actions by Priority and Action party: a report with all actions sorted by Priority, opens in
           Word
       •   Actions of a specific Action party: a report with all actions for a specific job title, opens in
           Word




                                         Figure 126 – Action report overview


       Note: If you are using BowTieXP in combination with IncidentXP, actions found in these reports can be a
       combination of bowtie actions and incident related actions (i.e. recommendations).




   10
   LINKING INCIDENTS WITH BOWTIE
       Note #1: To create templates containing bowtie information, or to link incident information to bowties, you will
       need an IncidentXP + BowTieXP license.




© WOLTER S KLUWER | CG E R isk 2021                INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                  PAGE 74
                                                          MANUAL
       Note #2: For a brief summary about the bowtie risk assessment methodology, see section 15,
       A brief description of a subset of bowtie methodology. Please refer to our BowTieXP manuals for an extensive
       explanation of the bowtie methodology and the BowTieXP software.


   The IncidentXP software allows you to have a full integration of incident and risk analysis, by using
   and linking existing risk assessment information (bowties) to your incidents.

   If relevant bowtie diagrams are available to use in your incident investigation, you can take entire
   scenario lines from the bowtie and pin them directly into your incident analysis diagram. This results
   in a better fit between incident and risk assessment analysis, which in turn allows you to improve the
   (initial) risk assessment.

   This allows organizations to analyze incidents in a barrier-based framework with minimal bowtie
   training. Creating a barrier-based incident diagram requires training. However with this templated
   software approach, all incident scenarios which already exist on bowties can be quickly plotted and
   analyzed using any licensed incident analysis method.

   After completing the incident analysis, the incident data and recommendations can be linked back to
   the bowtie risk assessment and visualized on existing barriers. Plotting all your incidents data onto a
   single BowTieXP case file allows you to aggregate barrier failure information and allows users to
   perform incident trend analysis and visualize weak spots within your management system.

   In short, the linking process allows you to gauge barrier effectiveness and availability on your bowtie
   model(s), based on real-world information from incident analyses.




                             Figure 124 - Bowtie risk assessment & incident analysis


   10.1 CREATING A BO WTIE TEMPLATED INCI D ENT ANALYSIS
   DIAGRAM
   In order to start the incident analysis using specific bowtie diagrams as template you have to:
       1. Make sure there are bowtie diagrams available in your current case file (.btf).
       2. Create an incident in the tree view. See section 3.2, Creating an incident & incident analysis
           diagram.




© WOLTER S KLUWER | CG E R isk 2021              INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                              PAGE 75
                                                        MANUAL
   If your bowtie case file contains bowtie information, the ‘Add Event from bowtie’ button will be
   enabled:




                                     Figure 125 - Add event from bowtie button


   By selecting the ‘Add Event from bowtie’ button, you will be presented with a drop-down menu
   allowing you to choose a bowtie element (top event, threat or consequence) to start from:




                                    Figure 126 - Add event from existing bowtie


       Note: The link between the incident analysis and the bowtie diagram is automatically created when the incident
       analysis diagram is created by using existing bowties.


   In this example we choose a road safety bowtie and have selected the threat ‘Blowout (tire)’. This
   event then will be added to the diagram. When usually hovering over the green button to add
   another event, we get another small orange button allowing to choose the next logical step in the
   (bowtie) event flow:




                                   Figure 127 - Add new (existing bowtie) event
   Once selected, not only is the top event ‘Losing control over the vehicle’ brought into the diagram,
   but also its associated barriers. In the same way you can continue adding every bowtie item needed.
   It is also possible to add items manually.




© WOLTER S KLUWER | CG E R isk 2021               INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                PAGE 76
                                                         MANUAL
                     Figure 128 - Added new (existing bowtie) event, including all barriers


   In using the Tripod Beta methodology, you are forced to alter some events manually (see section 5.2,
   Creating a Tripod Beta diagram ). Tripod Beta uses trio’s instead of straight event lines, therefore you
   will need to rework your events methodically. Each (threat) event must be split up into two
   elements: an agent and an object. Because of this you will probably need to replace some of the
   barriers, according to the correct event lines. You can do this by ‘Cut’ (selecting a barrier and press
   Ctrl + X) and ‘Paste’ (selecting the alternate connector and press Ctrl + V).

   In all methods, after creating the diagram, every barrier needs to be assessed and assigned a barrier
   state (failed, missing, unreliable, inadequate or effective), and provided with a causation assessment.
   For each method specifically, see:

       •   BSCAT, see section 4.5, Add a BSCAT causation assessment.
       •   Tripod Beta, see section 5.5, Add a Tripod Beta causation assessment.
       •   BFA, see section 6.5, Add a BFA causation assessment.


   10.1 .1 D ifference in detail level between risk
   assessment and incident analysis
   There exists a mismatch in the perspective/ abstraction level of the defined barriers between the risk
   analysis (bowtie diagrams) and the incident analysis. Barriers in incident analyses tend to be
   described more specific than those in risk analysis – those tend to be defined more abstract, high-
   level. For example: ‘high level alarm’ in a risk assessment can become (more specific) ‘High level
   alarm A2.1 on Tank X’ in your incident analysis. Incident analysis allows you to be more specific
   because -once an incident occurred- specific information from that situation is available, whereas in
   risk assessment all scenarios need to be covered.


       Note: The name of the elements in your incident analysis diagram can be adjusted without losing the connection
       with the linked bowtie diagram, nor altering its content.



   10.2 LINK ING INCID ENT ANALYSIS ELEMENTS TO BO WTIE
   DIAGRAMS
   The link between the incident diagram and a particular bowtie can also be created after finishing a
   regular incident diagram. To link an incident barrier (or an event if you are using the TOP-SET/RCA
   method) to a bowtie barrier, double click an incident barrier and go to the ‘Barriers’ tab in the editor.




© WOLTER S KLUWER | CG E R isk 2021               INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                                PAGE 77
                                                         MANUAL
                         Figure 129 - Linking an incident barrier to bowtie barrier


   Select the corresponding bowtie barrier(s) within the ‘Available’ window and move it to the
   ‘Assigned’ window by either dragging and dropping or using the arrows in between. The filter can be
   used to search for a specific barrier.

   The process of linking an incident event to a bowtie event (threat, top event or consequence) works
   almost the same. Again, double click the incident event and this time search for a corresponding
   bowtie event in the tabs Hazards, Threats, Consequences and Escalation factors.




                          Figure 130 - Linking an incident event to a bowtie event


   Select the corresponding bowtie element(s) within the ‘Available’ window and move it to the
   ‘Assigned’ window by either dragging and dropping or using the arrows in between. The filter can be
   used here as well.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                               PAGE 78
                                                   MANUAL
   10.3 V ISUALIZING INCID ENT ANALYSIS D ATA O N TH E
   BO WTIE D IAGRAM
   The barriers failure statistics and failure details can be shown on the bowtie diagram once the links
   between the bowtie barriers and the incident barriers have been made. Failure details include the
   barrier failure state and the causation paths. To enable this view, you select the incident filter from
   the filters drop-down on the bowtie diagram toolbar:




                                    Figure 131 - Enabling the Incident Filter


   Here is a cut out of a bowtie diagram showing the incident barrier states:




                          igure 132 - Bowtie diagram showing incident barrier states


   10.3 .1          Incide nt F ilte r toolbar
   Once the incident filter is selected, the incident filter toolbar will be displayed. The filter toolbar
   allows you to fine-tune what information will be shown. You can choose to show data from one
   incident, all incidents or from a specific set of incidents:




                                       Figure 133 - Incident filter toolbar


   In ci d en t s
   You can choose which incident(s) you would like to display within the ‘Incidents’ selection screen.




© WOLTER S KLUWER | CG E R isk 2021            INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                       PAGE 79
                                                      MANUAL
                               Figure 134 - Incident filter (incident selection)


   General
   You can change various general visibility settings, underneath the option ‘General’:

       •   Show Incident count: show the number of incidents the barrier was linked to.
       •   Show Failure Percentages: show the percentual barrier failure from the incidents it was
           linked to.
       •   Show linked item count: show the number of incident shapes the bowtie element was linked
           to. As incident diagrams tend to be more specific (where bowties are more general/abstract),
           it can happen that a single bowtie barrier will be linked to multiple incident barriers within
           one incident.
       •   Show item state count: show the number of counts of the defined status within your
           incident diagrams (e.g. failed, inadequate, unreliable, missing, or effective). See figure 135
           for an example.
       •   Include draft Incident Diagrams: this can be used to in- or exclude draft incident diagram
           data onto the bowtie diagrams. The incident diagram status can be altered (between
           ‘Working Draft’ and ‘Completed’) by double clicking an incident diagram in the tree view, or
           within the Incident Manager.




                                Figure 135 - Incident filter (general settings)


   BSCAT/BFA
   You can change various BFA/BSCAT diagram related visibility settings, underneath the option
   ‘BSCAT/BFA’:

       •   Show links: whether or not to show the BSCAT/BFA chart values.
              o Show top level links only: visualize the actual item, or only the main category.
              o Show codes only: visualize the full description, or just the items’ code.
       •   Show free text: whether or not to show the free text description of the causation path. For
           BFA, it will also show the name and count of the causation taxonomy.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 80
                                                   MANUAL
                                Figure 136 - Incident filter (BSCAT/BFA settings)


   Tripod
   You can change various Tripod diagram related visibility settings, underneath the option ‘Tripod’:

       •   Show Tripod causations: whether or not to show Tripod Causation paths.
              o Only show BRF summary: visualize the entire causation path or only the BRF codes.




                                  Figure 137 - Incident filter (Tripod settings)


   Da te & Orga n i za ti o n a l Un i t
   In larger organizations, it can be helpful to compare the incident statistics of different sites and
   dates. This allows a company to benchmark within their own organization to decide where additional
   efforts are required. To make these comparisons easier, a ‘Date’ and ‘Organizational Unit’ filter are
   implemented.

   For the date filter within the incident filter, it is possible to use the predefined date options, or to
   define a custom date range.




                                   Figure 138 - Incident filter (date settings)




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                       PAGE 81
                                                     MANUAL
   To filter on Organizational Units, (un)check all relevant boxes. Dark green fields indicate that there
   are items present linked to this specific organizational unit. Light green means there is an indirect
   link, and white/ no color means there are no incident items linked to this organizational unit.




                          Figure 139 - Incident filter (Organizational Unit settings)


   This makes it easier to discover which organizational units contain relevant incident information.


   10.3 .2         F ilter strength
   You can choose to show all bowtie elements or limit the bowtie to only the paths involved in the
   incident(s). This can be arranged through the filter strength:




                                          Figure 140 - Filter strength


   The filter strength has six strength levels:
       1. Show everything
       2. Show everything + grey out non selected items
       3. Hide unrelated paths
       4. Hide unrelated paths + grey out non selected items
       5. Show only directly and indirectly selected
       6. Show only directly selected + grey out indirectly selected




   11
   CONFIGURING THE DIAGRAM
   IncidentXP has diagrams which are configurable in many ways – there are multiple options to tailor
   the display to your specific needs. You can apply various colors and show all kinds of extra
   information. This chapter will explain all the various settings.



© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 82
                                                    MANUAL
   11.1 CH ANGING D ISPLAY SETTINGS F O R A SIN GLE TYPE O F
   ITEM
   In the diagram, if you hover your mouse over an item, an eye-shaped icon will appear:




                                Figure 141 - Eye shaped icon - display settings


   By clicking the eye-shaped icon, a dialog box will appear which allows you to customize the display of
   the item. It lists all the items can be shown on the diagram as additional information box below the
   element. Choose from this list by selecting the radio button on the S or L column. S stands for Short
   format and L stands for Long format.




                             Figure 142 - Set incident barrier display dialog (1/2)


   You can also select which property is to be used to color the shape itself – this is done by selecting a
   check box in the color column:




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 83
                                                    MANUAL
                            Figure 143 - Set incident barrier display dialog (2/2)


   As an example, the above settings result into the visualization shown below:




            Figure 144 - Incident barrier colored by barrier state and displaying document links


   You can adjust the display for all elements on the diagram. If you need to adjust overall display
   configurations, you can also use the overall diagram display options dialog, discussed in the next
   section.


   11.2 CH ANGING D ISPLAY SETTINGS F O R TH E WH O LE
   DIAGRAM
   Changing settings for the entire diagram is done by clicking the ‘large eye-icon’ in the toolbar (  ).
   This shows you the current display settings. The dialog consists of several tabs which allow you the
   change the appearance of the diagram(s) shapes.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                   PAGE 84
                                                    MANUAL
                               Figure 145 - Diagram display options – ‘Show’


   11.2 .1         D isplay se ttings - the ‘Show’ tab
   The first tab ‘Show’ is about what additional information to display on diagrams and whether and
   how to display actions and causation descriptions. On the left-hand side you can choose whether to
   show actions, and if so, which ones. You can also select here whether and how to display the
   causation descriptions.




                 Figure 146 - Diagram display options - actions and causation descriptions


   On the right-hand side, you can choose per element which information to display and again whether
   to use a Short or a Long format to do so.




                    Figure 147 - Diagram display options - right hand side of ‘Show’ tab


   11.2 .2         D isplay se ttings - the ‘Colors’ tab
   The next tab within the overall display option is on if and how to color the shapes.



© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                               PAGE 85
                                                    MANUAL
                               Figure 148 - Diagram display options – ‘Colors’


   On the left-hand side of this tab we can configure the action shapes’ color, and if we want colors to
   appear in the additional information boxes. If some colors lead to bad readability of the information,
   we can also choose to display its color only in the left corner of the information box (option ‘Show as
   flag next to text’).

   In the right hand side of this tab we can select which property to use to color the element itself.


   11.2 .3         F onts to use – the ‘F onts’ tab
   The next tab allows us to set default fonts. Click one of the main buttons to bring up a font dialog. An
   example of the current font will be shown in the box. The General Font is shown in the main incident
   diagram shapes (events and barriers). The Extra Info Font is shown for additional information.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 86
                                                    MANUAL
                                Figure 149 - Diagram display options – ‘Fonts’


   11.2 .4         F onts to use – the ‘Sizing & Layout’ tab
   The last tab allows us to make decisions about sizing and layout of boxes and connectors.




                          Figure 150 - Diagram display options – “Sizing & Layout”


   11.3 AD JUSTING LO O K UP TABLES
   While creating incident diagrams (and the timeline), you will find a lot of drop-down pick lists. These
   lists almost always refer to the Lookup Table lists in the tree view. These lookup tables are
   configurable to your companies’ standards.

   Right-click a lookup table node to add, edit or delete a value in the list or change the value order.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 87
                                                    MANUAL
                                  Figure 151 - Adjusting the lookup tables


   11.3 .1         Adjusting lookup table colors
   The colors used in the diagram are also adjustable. They can be set using the same editor in the tree
   view. Find the appropriate value to edit in the tree view, open the editor and subsequently select
   ‘Choose’ to select a color and pattern for the respective value.




                                 Figure 152 - Adjusting lookup table colors


   Within this sub-editor we can thereafter choose a pattern and/or a color. Use the Background color
   to adjust the coloring of the additional information boxes within the diagram.




© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                 PAGE 88
                                                   MANUAL
                                        Figure 153 - Color picker dialog


   The software has a default color that is used when no value is assigned to an item. This color can also
   be adjusted: Select Case > ‘Edit null value colors’ in the upper menu toolbar. This will bring up the
   dialog below Clicking an item will again bring up a color picker dialog.




                                         Figure 154 - Null value colors


   11.4 LAYOUT GRID
   IncidentXP has different layout options to facilitate incident analysis diagram creation and
   development. There are two main grid layout options:
       • Assisted Layout: the incident diagram shapes fit precisely into a grid in the diagram space.
           BFA & BSCAT diagrams have this option enabled by default. The shapes will automatically
           snap to the grid when adding or dropping shapes.
       • Free grid Layout: the diagram shapes can freely be distributed across the diagram space by
           moving them around or with cutting and pasting actions. The free layout puts the user in full
           control over the layout of the diagram when rearranging the shapes. This option is especially
           useful for non-linear incident analysis diagrams. RCA & Tripod-Beta diagrams have this
           option enabled by default.

   To select ‘Assisted Layout’ or ‘Free grid Layout’, use this icon in the toolbar.




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 89
                                                     MANUAL
                                       Figure 155 - Layout options icon


   When this icon is selected, various options are presented. To select a main layout option, pick the
   assisted or free layout option.




                                Figure 156 - Select assisted or free grid layout


   In the grid options drop-down, there are various other grid display options, such as showing/hiding
   the grid, or showing the grid as lines as dots.

   Below the grid options menu it is possible to select ‘Perform Automatic Layout’. This can be used
   when you have been creating a diagram using the free grid layout, but you want the shapes to snap
   to the grid to make them more readable. This is because shapes can have the possibility (until a
   certain extend) to overlap when using the free grid layout.




   12
   CASE FILE TOOLS
   12.1 LISTV IEW WIND O W
   Go to View > Windows > Listview. The listview window sums up underlying information of a selected
   tree view node. The listview is especially useful for the display of backlinks.

   When a document link is referenced on say, a barrier, there’s a virtual arrow pointing from the
   barrier to the location in the tree view where the document link is defined. This arrow is called the
   link. We can also follow this arrow in the reverse direction, from the document links to all the
   barriers where it is used. These are called the backlinks. The backlinks are a summation of all places
   where the item is used.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                   PAGE 90
                                                    MANUAL
   A similar concept applies to lookup table values, the items used in drop-down lists such as the
   incident type. For each item we can find out where they are used by revealing their backlinks. This is
   done by right-clicking the item of which you want to see where it is referenced, and selecting the
   option in the context menu called ‘Show usage references’. The listview will show the usage
   references and indicate that it is displaying backlinks.


   12.2 QUALITY CH ECK S WIND O W
   The quality checks window can display a number of different reports pertaining to the quality and
   completeness of your incident analysis. You can find the quality checks hidden on the right side of
   the diagram in the default layout or open the window by selecting View > Windows > Quality Checks.

   From the drop-down box at the top of the window, select the report you want from the list. Once
   you have selected a report, the window is populated with its results. To refresh the list of items, click
   the button with the blue circular arrow. The quality checks are extremely helpful in identifying
   missing data in your analysis.




                                     Figure 157 - Quality checks window


   12.3 CASE OVERVIEW
   The case file overview presents many views of the data in your case file from different perspectives,
   allowing visualization of relationships which might otherwise be less evident, plus allowing you to
   quickly find related data.

   The Case overview is a separate tab in the diagram window. Activation of this tab can be done
   through the main menu selecting View > Windows > Case overview.

   Using the drop-down menu on the toolbar labeled Overview Perspectives, you can change the
   overviews’ perspective.




                                     Figure 158 - Case overview window




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 91
                                                    MANUAL
   All output from each overview can be exported to Excel by means of the ‘Export To Excel’ button (
       ).

   When information in the case file changes, the case file overview will update automatically. If, for
   some reason, you want to rebuild the output you can do so with the ‘reset/reload’ button (         ).

   All the case overview buttons are shown below.



    Icon                Description

                        Execute expand by keyword operation by means of the keywords entered
                        into the text box

                        Go to previous keyword match

                        Go to next keyword match

                        Expand all

                        Collapse all

                        Go to definition in the tree view

                        Show relationship model of selected entity (only activated in combination
                        with BowTieXP)

                        Reset/reload

                        Export to Excel


   12.3 .1         D ocume nt Links pe rspe ctive
   This perspective lists all document links defined in your case file and where they are used.

   12.3 .2         Wiki Pages perspective
   This perspective lists all Wiki groups and Wiki pages defined in your case file and provides a full
   description of them.

   12.3 .3         Incide nt A ctions pe rspe ctive
   This perspective lists all actions present on any incident analysis diagram and displays them in two
   groups: open actions and completed actions.

   12.3 .4         IncidentXP Incident Analysis O verview
   This perspective lists aggregated causation chart value counts, which enables performing trend
   analysis. It aggregates the values for the entire case file as well as per incident analysis diagram.




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                     PAGE 92
                                                     MANUAL
              Figure 159 - Case overview – perspective: IncidentXP incident analysis overview




   13
   LINKING TO DOCUMENTATION
   13.1 INTRO D UCTI O N
   A lot of information which is present in a case file refers to things which have already been
   documented elsewhere, and in a lot of places in a case file you might want to refer to that
   information. Besides that, evidence is often captured in a separate document, which should be linked
   to the incident analysis as well.

   To solve this need, both BowtieXP and IncidentXP offers a document links module. Document links
   are pointers to external documentation which can be linked to various elements within your case file.
   In this chapter you will learn how to make efficient use of them.

   Document links are defined in a central hierarchy within the tree view, and are then linked to all
   relevant (diagram) items throughout the rest of the case file. When assigned they can be shown on
   the diagram, and quickly accessed by means of the ‘open/go to’ icon     :




                  Figure 160 - Barrier showing documents links along with "open" button




© WOLTER S KLUWER | CG E R isk 2021        INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                PAGE 93
                                                  MANUAL
   13.2 CREATING A D O CUMENT LINK
   To create a document link, navigate to the ‘Document Link Groups’ node in the tree view, right-click
   this node and select ‘New Document Link Group’. Within a Document Link Group, actual Document
   Links can be created. Document links can be created in a hierarchical structure. A sub-document link
   will be created if you add a Document Link by right-clicking an existing Document Link or Document
   Link Group.




                                 Figure 161 - Add new document link (1/3)




                                 Figure 162 - Add new document link (2/3)


   A document link contains a code, a name, a description and a location. The location defines where
   the actual document resides. This can be either a URL or a file path.




© WOLTER S KLUWER | CG E R isk 2021        INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                PAGE 94
                                                  MANUAL
                                  Figure 163 - Add new document link (3/3)


   13.3 LINK ING A D O CUMENT LINK TO TH E D IAGRAM
   After defining document links, they can be linked onto other elements by the usual methods of
   dragging and dropping (either from the tree view or within the editor box shown below) or using the
   arrows within the document links tab in the editor window.




                 Figure 164 - Linking a document link to an incident event (two alternatives)




   14
   GETTING DATA OUT OF THE PROGRAM
   14.1 INTRO D UCTI O N
   Putting information into case files is useful, but it is even better to also have the opportunity to get
   all the information out of the case files in various perspectives and formats, in order to share them
   with your desired audience.

   In this chapter you will learn all the different methods for outputting information. We will discuss:
        • The output of the various reports,
        • The export of the diagrams.




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                     PAGE 95
                                                     MANUAL
   14.2 REPO RTING
   Besides being able to create a specific report of a certain incident analysis using the report function
   within the Incident Manager, you can also report information from an entire case file in various ways
   using the built-in reporting engine. There are various reports available with different functionality
   and visualization of the information. Depending on the chosen report, the format will either be
   Microsoft Word or Microsoft Excel.

   14.2 .1         Ge ne rating a re port
   Follow these steps to generate a report:

       1. Select Tools → Reports, or click the report button in the toolbar (     ).
       2. Select the desired report.
       3. After selecting the report, it’s automatically opened in Word or Excel.

   The dialog box depicts already in advance which reports are offered as an Excel file. Deductively the
   other reports will be generated in Word.

   After running a report you can thereafter save the report under a name of your choosing.




                                      Figure 165 - Generating a report

   14.2 .2         Re port from te mplate
   It is possible to define a company template report, complete with logos, fonts and other styling and
   fill that with information from an incident analysis, using tags that are inserted into the template.

   To create a custom template, follow these steps:
       1. Open Word (for this example, we will use Word 2013).
       2. Go to File > Options > Customize Ribbon.
       3. Make sure the “Developer” option is ticked in the Main Tabs section.




© WOLTER S KLUWER | CG E R isk 2021           INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 96
                                                     MANUAL
                               Figure 166 - Microsoft Word - Customize Ribbon


      1. Open the Developer tab. You can add fields with the “Aa” icon. The tooltip describes it as a
         “Plain Text Content Control”.




                          Figure 167 - Microsoft Word - Plain Text Content Control


      2. The field will look like this:




                        Figure 168 - Microsoft Word - Plain Text Content Control (2)


      3. To make sure the field is filled with the right data, select the field and click “Properties” in
         the ribbon.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                    PAGE 97
                                                    MANUAL
                         Figure 169 - Microsoft Word – Content Control Properties


      4. In the “Tag” field, you can insert one of the tags that are available (see Table below). If you
         want to see these tags in the document, you have to turn on the “Design Mode”.

   Available tags:
       • Incident.Code
       • Incident.Description
       • Incident.LongDescription
       • Incident.WhatHappened
       • Incident.DateAndTime
       • Incident.DateAndTimeTimeZoneId
       • Incident.DateAndTimeFormat
       • Incident.IncidentType
       • Incident.IncidentCategory
       • Incident.IncidentTier
       • Incident.OrgUnit
       • Incident.Latitude
       • Incident.Longitude
       • Incident.PlotPlan
       • Incident.PPL_X
       • PPL_Y
       • PPL_S
       • IncidentDiagram.DiagramStatus
       • IncidentDiagram.WorkingDraftStatusDescription
       • IncidentDiagram.CompletedStatusDescription
       • IncidentDiagram.TopSetZone1a
       • IncidentDiagram.TopSetZone1b
       • IncidentDiagram.TopSetZone2
       • IncidentDiagram.TopSetZone3
       • IncidentDiagram.TopSetZone4
       • IncidentDiagram.Name
       • IncidentDiagram.MethodologyType
       • IncidentDiagram.UseTopologicalLayout
       • IncidentDiagram.IsSIRDiagram
       • IncidentDiagram.ShowZones
       • IncidentDiagram.ShowConnectorLabels
       • TimelineDiagram.TimelinePictures
       • IncidentDiagram.TreeOverview
       • IncidentDiagram.TreeCutUp
       • IncidentDiagram.Counts
       • IncidentDiagram.Actions
       • IncidentDiagram.DocumentLinks
       • IncidentDiagram.AllL1Causes
       • IncidentDiagram.AllL2Causes
       • IncidentDiagram.AllL3Causes



© WOLTER S KLUWER | CG E R isk 2021         INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 98
                                                   MANUAL
       •   IncidentDiagram.AllEndEvents
       •   IncidentDiagram.PotentialEndEvents
       •   IncidentDiagram.NonPotentialEndEvents
       •   TimelineDiagram.EventsTable
       •   Application.BowTieXPVersion
       •   Incident.PlotPlanImage
       •   Incident.GeoLocationImage

   Add the tags to the Content Control Properties:




                       Figure 170 - Microsoft Word – Content Control Properties (2)


       5. Save your template and close Word. You cannot run the report while it is opened in Word.
       6. In IncidentXP, go to Tools > Reports > Incidents > Incident from template.




                                Figure 171 - Incident report from template




© WOLTER S KLUWER | CG E R isk 2021        INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                             PAGE 99
                                                  MANUAL
                                  Figure 172 - Selecting reporting template


       7. Make sure “Use built-in template” is deselected and click “Browse” to select your template.
       8. Click OK to run the report.

   You can edit and change the report, then update the tags by selecting the same word file again (step
   9). All fields will then be updated, without changing the information around it.


   14.3 EXPORT OF DIAGRAMS
   There are four ways to export an incident analysis diagram (or timeline) in isolation. Find the buttons
   in your diagram toolbar to see which one to use when:


    Icon              Description

                      Copy to clipboard: allows you to paste the diagram in another application
                      (e.g. PowerPoint). This will export a vector based image, which can be
                      enlarged and reduced in size without losing the image’s quality. This
                      makes it very suitable for documents, printing or presentations.

                      Save diagram as…: allows you to save the diagram as a picture (e.g. jpg).
                      Most formats are not vector based and therefore not ideal for printing.

                      Export as svg drawing: allows you to export picture and save it as an XML
                      based scalable vector graphic.


                      Export to Microsoft Visio: allows you to export to Microsoft Visio.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 100
                                                    MANUAL
   15
   A BRIEF DESCRIPTION OF A SUBSET OF
   BOWTIE METHODOLOGY
   The bowtie methodology is used for risk assessment, risk management and risk communication. The
   method is designed to give a better overview of the situation in which certain risks are present; to
   help people understand the relationship between the risks and organizational events.

   Risk in the bowtie methodology is represented by the relationship between hazards, top events,
   threats and consequences. Barriers are used to display what measures an organization has in place to
   control the risk.

   All these are combined in an easy-to-read diagram as follows:




                                        Figure 173 - A bowtie diagram


   15.1 HAZARD
   The word “hazard” suggests that it is an unwanted entity, but in fact it is the opposite: it is exactly
   the thing you want or even need in order to create business. It is an entity with the potential to cause
   harm but without it there is no business. For example in the oil industry; oil is a dangerous substance
   (and can cause a lot of harm when treated without special care) but it is the one thing that keeps the
   oil industry in business! When a hazard is managed or under control, it can be of no harm.


   15.2 TO P EV ENT
   As long as a hazard is controlled it is in its desired state. For example: oil is transported through a
   pipe on its way to shore. Certain events can cause the hazard to be released however. In bowtie
   methodology such an event is called a top event. The top event is not a catastrophe yet, but the
   dangerous characteristics of the hazard are now released. This is the moment in which control over
   the hazard is lost. For example: oil is spilling outside of the pipeline (loss of containment). Not a
   major disaster yet, but if not mitigated correctly it can result in multiple undesired events
   (consequences).




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 101
                                                    MANUAL
   15.3 TH REATS
   Often there are several factors that could cause the top event to occur. In bowtie methodology these
   are called threats. These threats need to be independently able to cause the top event. For example:
   corrosion of the pipeline can lead to the loss of containment.


   15.4 CONSEQUEN CE S
   When a top event has occurred it can lead to certain consequences. A consequence is a potential
   event resulting from the release of the hazard, which entails damage, loss, or injuries/fatalities.
   Consequences in bowtie methodology are therefore unwanted events that an organization ‘by all
   means’ wants to avoid. For example: oil leaking into the environment.


   15.5 BARRIERS (ALSO K NO W N AS CO NTRO LS)
   Risk management is about controlling risks. This is done by placing barriers on bowtie scenarios to
   prevent certain events form happening. A barrier (or control) can be any measure taken to prevent
   arriving at un undesired state.

   In bowtie methodology there are proactive barriers (on the left side of the top event) that prevent
   the top event from happening. For example: regularly corrosion inspections of the pipelines. There
   are also reactive barriers (on the right side of the top event) that prevent the top event resulting into
   unwanted consequences. For example: leak detection equipment or concrete floor around oil tank
   platform.

   Note the terms barrier and control are the same construct and depending on industry and company,
   one or the other is used. Barrier is the defaulted term in our software tools.


   15.6 ESCALATIO N F ACTO RS/D EF EA TI NG F ACTO RS/BARR IE R
   DECAY MECHANISMS
   In an ideal situation a barrier will stop a threat from causing the top event. However, many barriers
   are not a 100% effective. There are certain conditions that can make a barrier fail. In bowtie
   methodology these are called escalation factors. An escalation factor is a condition that leads to
   increased risk by defeating or reducing the effectiveness of a barrier.

   Escalation factors are also known as defeating factors or barrier decay mechanisms – which term is
   used is dependent on industry and company. The term escalation factor is defaulted in our software
   tools.


   15.7 TERMINO LO GY RECAP
   The following terms should now be familiar to you:
       • The hazard, part of normal business but with the potential to cause harm, can be released
           by:
       • A top event, no catastrophe yet but the first event in a chain of undesired events.
       • The top event can be caused by threats (sufficient and independent causes).
       • The top event has the potential to lead to unwanted consequences.
       • (Proactive) barriers are measures taken to prevent threats from occurring or resulting into
           the top event.
       • (Reactive) barriers are measures taken to prevent that the top event leads to unwanted
           consequences, or mitigates the effects from those consequences.




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 102
                                                    MANUAL
       •   An escalation factor is a condition that defeats or reduces the effectiveness of a barrier.




   16
   SUPPORT
   16.1 HELPDESK
   There is a helpdesk for users that have bought an IncidentXP license with a valid support and
   maintenance contract. This helpdesk assists people that have technical and user-related questions
   regarding IncidentXP.

   You can reach the IncidentXP helpdesk via e-mail at support@cgerisk.com or by telephone on +31 (0)
   88 1001 350.

   Please refer to https://www.cgerisk.com/support/ for our Support & Maintenance overview. It
   explains what customers receive when purchasing support & maintenance.


   16.2 SO F TWARE TRAINING
   Please contact us at support@cgerisk.com or by telephone on 31 (0) 88 100 1350 for software
   training.


   16.3 BSCAT METH O D TRAINING
   DNV-GL offers training courses in accident investigation using the BSCAT method. If you are
   interested in such training, please contact:

   Mr. Selva Subramaniam
   Email: Selva.Subramaniam@dnvgl.com
   Phone: +442038165763
   Highbank House, Exchange Street, Stockport,
   Cheshire SK3 0ET, United Kingdom


   16.4 TRIPO D BETA METH O D TRAINING
   Please contact the Energy Institute at tripod@energyinst.org. An overview of all Tripod Beta trainings
   can be found here: https://publishing.energyinst.org/tripod/training


   16.5 TO P-SET METH O D TRAINING
   TOP-SET method training is available from:

   Kelvin TOP-SET Ltd
   Annickbank Innovation Campus
   Irvine KA11 4LF
   Scotland, United Kingdom
   +44 (0) 1475 560 007




© WOLTER S KLUWER | CG E R isk 2021          INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                                                                  PAGE 103
                                                    MANUAL
   enquiries@kelvintopset.com
   http://www.kelvintopset.com/




© WOLTER S KLUWER | CG E R isk 2021   INC IDENTXP S OFTWARE
CONF IDENTIAL AND PR OPR IETAR Y                              PAGE 104
                                             MANUAL
